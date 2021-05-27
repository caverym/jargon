#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{
    Result,
    Error,
    ErrorKind,
};

/// Main Jargon struct which represents the user's program. This structure will contain the program
/// name, author (optional), version (optional), keys (arguments to parse), and actual command line
/// arguments.
///
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Jargon {
    name: String,
    author: Option<String>,
    version: Option<String>,
    keys: Keys,
    args: Vec<String>,
}

impl Jargon {
    /// # Jargon::new()
    ///
    /// creates a new instance, or app, using the `Jargon` struct. For more information on the
    /// struct, read its documentation. This initializes basic information needed to run.
    ///
    /// ## Usage
    ///
    /// basics:
    /// ```
    /// fn main() {
    ///     let app: jagon::Jargon = jargon::Jargon::new("basic app", std::env::args().into());
    /// }
    /// ```
    ///
    /// with author and version and keys:
    /// ```
    /// fn main() {
    ///     let app: jargon::Jargon = jargon::Jargon::new("app_add", std::env::args().into())
    ///         .author("avery")
    ///         .version("0.1.0")
    ///         .add(["-a", "--add"]);
    /// }
    /// ```
    ///
    pub fn new<T: ToString>(name: T, args: Vec<String>) -> Jargon {
        Jargon {
            name: name.to_string(),
            author: None,
            version: None,
            keys: Keys::new(),
            args,
        }
    }

    pub fn author<T: ToString>(mut self, author: T) -> Jargon {
        self.author = Some(author.to_string());
        self
    }

    pub fn version<T: ToString>(mut self, version: T) -> Jargon {
        self.version = Some(version.to_string());
        self
    }

    pub fn add<T: Into<Key>>(mut self, arg: T) -> Jargon {
        let mut v: Vec<Key> = vec![arg.into(); 1];
        self.keys.0.append(&mut v);
        self
    }

    pub fn arg_bool<T: Into<Key>>(&self, keys: T) -> bool {
        let key: Key = keys.into();

        if let Some(k) = key.short {
            if self.args.contains(&format!("-{}", k)) {
                return true;
            }
        } else if let Some(k) = key.long {
            if self.args.contains(&format!("--{}", k)) {
                return true;
            }
        }
        false
    }

    pub fn get_key<T: ToString>(&self, key: T) -> Option<Key> {
        let key: String = key.to_string();
        for k in self.keys.0.iter() {
            if k.name == key {
                return Some(k.to_owned());
            }
        }
        None
    }

    pub fn option_arg_str<T: ToString>(&self, key: T) -> Option<String> {
        let key: Key = if let Some(k) = self.get_key(key) {
            k
        } else {
            return None;
        };

        let long: String = format!("--{}", key.get_long());
        let short: String = format!("-{}", key.get_short());

        for i in 1..self.args.len() {
            if self.args[i] == long || self.args[i] == short {
                return Some(self.args[i+1].to_owned())
            }
        }

        None
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Keys(Vec<Key>);

impl Keys {
    pub fn new() -> Keys {
        Keys(Vec::new())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Key {
    name: String,
    short: Option<char>,
    long: Option<String>,
}

impl Key {
    pub fn new<T: ToString>(name: T) -> Key {
        Key {
            name: name.to_string(), short: None, long: None
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_short(&self) -> String {
        self.short.unwrap_or('\0').to_string()
    }

    pub fn get_long(&self) -> String {
        self.long.to_owned().unwrap_or("\0".to_string()).to_string()
    }

    pub fn short<T: Into<Key>>(mut self, short: T) -> Key {
        let short: Key = short.into();
        self.short = short.short;
        self
    }

    pub fn long<T: Into<Key>>(mut self, long: T) -> Key {
        let long: Key = long.into();
        self.long = long.long;
        self
    }
}

impl From<String> for Key {
    fn from(s: String) -> Key {
        let mut short: Option<char> = None;
        let mut long: Option<String> = None;

        if let Some(pre) = s.strip_prefix("--") {
            long = Some(pre.to_string())
        }

        if let Some(pre) = s.strip_prefix("-") {
            let ch: Vec<char> = pre.chars().collect();
            if ch.len() == 1 {
                short = Some(ch[0])
            }
        }

        let name: String = if let Some(a) = &long {
            a.to_string()
        } else if let Some(a) = short {
            a.to_string()
        } else {
            "".to_string()
        };

        Key {name, short, long }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        s.to_string().into()
    }
}

impl From<char> for Key {
    fn from(c: char) -> Key {
        Key {
            name: c.to_string(),
            short: Some(c),
            long: None,
        }
    }
}

impl From<[&str; 2]> for Key {
    fn from(keys: [&str; 2]) -> Key {
        let mut short: Option<char> = None;
        let mut long: Option<String> = None;

        keys.iter().for_each(|s| {
            if let Some(pre) = s.strip_prefix("--") {
                long = Some(pre.to_string())
            }

            if let Some(pre) = s.strip_prefix("-") {
                let ch: Vec<char> = pre.chars().collect();
                if ch.len() == 1 {
                    short = Some(ch[0])
                }
            }
        });

        let name: String = if let Some(a) = &long {
            a.to_string()
        } else if let Some(a) = &short {
            a.to_string()
        } else {
            "".to_string()
        };

        Key {name, short, long }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn impl_key_str_short() {
        let key: crate::Key = "-a".into();
        assert_eq!(
            crate::Key {
                name: "a".to_string(),
                short: Some('a'),
                long: None
            },
            key
        );
    }

    #[test]
    fn impl_key_str_long() {
        let key: crate::Key = "--all".into();
        assert_eq!(
            crate::Key {
                name: "all".to_string(),
                short: None,
                long: Some("all".to_string())
            },
            key
        );
    }

    #[test]
    fn impl_key_string_short() {
        let key: crate::Key = "-a".to_string().into();
        assert_eq!(
            crate::Key {
                name: "a".to_string(),
                short: Some('a'),
                long: None
            },
            key
        );
    }

    #[test]
    fn impl_key_string_long() {
        let key: crate::Key = "--all".to_string().into();
        assert_eq!(
            crate::Key {
                name: "all".to_string(),
                short: None,
                long: Some("all".to_string())
            },
            key
        );
    }

    #[test]
    fn impl_key_array_be() {
        let key: crate::Key = ["-a", "--all"].into();
        assert_eq!(
            crate::Key {
                name: "all".to_string(),
                short: Some('a'),
                long: Some("all".to_string())
            },
            key
        )
    }

    #[test]
    fn impl_key_array_le() {
        let key: crate::Key = ["--all", "-a"].into();
        assert_eq!(
            crate::Key {
                name: "all".to_string(),
                short: Some('a'),
                long: Some("all".to_string())
            },
            key
        )
    }

    #[test]
    fn app() {
        let args: Vec<String> = vec!["app".to_string(); 1];
        let app: crate::Jargon = crate::Jargon::new("app", args)
            .author("avery")
            .version("0.1.0");

        assert_eq!(
            crate::Jargon {
                name: "app".to_string(),
                author: Some("avery".to_string()),
                version: Some("0.1.0".to_string()),
                keys: crate::Keys(Vec::new()),
                args: vec!["app".to_string(); 1]
            },
            app
        );
    }

    #[test]
    fn app_add() {
        let args: Vec<String> = ["app_add".to_string()].to_vec();
        let app: crate::Jargon = crate::Jargon::new("app_add", args).add(
            crate::Key::new("add")
                .short("-a")
                .long("--add")
        );

        assert_eq!(
            crate::Jargon {
                name: "app_add".to_string(),
                author: None,
                version: None,
                keys: crate::Keys([crate::Key {name: "add".to_string(), short: Some('a'), long: Some("add".to_string())}; 1].to_vec()),
                args: ["app_add".to_string();1].to_vec(),
            },
            app
        )
    }

    #[test]
    fn bool_arg_short_t() {
        let args: Vec<String> = ["bool_arg_short".to_string(), "-a".to_string()].to_vec();
        let app: crate::Jargon = crate::Jargon::new("bool_arg_short", args).add(["-a", "--all"]);

        assert!(app.arg_bool("-a"));
    }

    #[test]
    fn bool_arg_short_f() {
        let args: Vec<String> = ["bool_arg_short".to_string()].to_vec();
        let app: crate::Jargon = crate::Jargon::new("bool_arg_short", args).add(["-a", "--all"]);

        assert!(!app.arg_bool("-a"));
    }

    #[test]
    fn bool_arg_long_t() {
        let args: Vec<String> = ["bool_arg_short".to_string(), "--all".to_string()].to_vec();
        let app: crate::Jargon = crate::Jargon::new("bool_arg_short", args).add(["-a", "--all"]);

        assert!(app.arg_bool("--all"));
    }

    #[test]
    fn bool_arg_long_f() {
        let args: Vec<String> = ["bool_arg_short".to_string()].to_vec();
        let app: crate::Jargon = crate::Jargon::new("bool_arg_short", args).add(["-a", "--all"]);

        assert!(!app.arg_bool("-a"));
    }

    #[test]
    fn keys() {
        let key: crate::Key = crate::Key::new("key")
            .short("-k")
            .long("--key");

        assert_eq!(
            crate::Key {
                name: "key".to_string(),
                short: Some('k'),
                long: Some("key".to_string()),
            },
            key
        )
    }

    #[test]
    fn arg_str() {
        let args: Vec<String> = [
            "arg_str".to_string(),
            "--add".to_string(),
            "1".to_string(),
        ].to_vec();

        let add: crate::Key = crate::Key::new("add").short("-a").long("--add");
        let app: crate::Jargon = crate::Jargon::new("arg_str", args)
            .add(add);

        assert_eq!(app.option_arg_str("add"), Some("1".to_string()));
    }
}
