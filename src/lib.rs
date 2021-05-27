/*!
# Jargon

A simple command line parser for Rust.

## Why?

There are many argument handlers/parsers for Rust, but they're all missing someting, or have too
much.

### Comparing Pico to Clap.

* Pico requires a lot of manual interaction with the arguments, not much abstraction. But I
definitely recommend it as the best there currently is.

* Clap… Clap has an idiotic amount of
abstraction, doing too much for its own good.

My goal in creating Jargon is to take ideas from both Pico and Clap to create a more helpful and
comprehensive argument parser.
*/

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{
    Result,
    Error,
    ErrorKind,
};

#[allow(missing_docs)]
macro_rules! notfound {
    ($e:expr) => {
        Err(Error::new(ErrorKind::NotFound, format!("argument '{}' not found", $e)))
    };
}

/// Main Jargon struct which represents the user's program. This structure will contain the program
/// name, author (optional), version (optional), keys (arguments to parse), and actual command line
/// arguments.
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
    /// use std::env::args;
    /// use jargon::Jargon;
    ///
    /// let app: Jargon = Jargon::new("basic app", args().collect());
    ///
    /// ```
    ///
    /// with author and version and keys:
    /// ```
    /// use jargon::Jargon;
    /// use std::env::args;
    ///
    /// let app: Jargon = Jargon::new("app_add", args().collect())
    ///     .author("avery")
    ///     .version("0.1.0")
    ///     .argument(["-a", "--add"]);
    /// ```
    pub fn new<T: ToString>(name: T, args: Vec<String>) -> Jargon {
        Jargon {
            name: name.to_string(),
            author: None,
            version: None,
            keys: Keys::new(),
            args,
        }
    }

    /// This method is used at the creation of a Jargon struct to add an app author.
    pub fn author<T: ToString>(mut self, author: T) -> Jargon {
        self.author = Some(author.to_string());
        self
    }

    /// Add an author name to an already existing Jargon struct.
    pub fn add_author<T: ToString>(&mut self, author: T) {
        self.author = Some(author.to_string());
    }

    /// Add an app version name to an already existing Jargon struct.
    pub fn add_version<T: ToString>(&mut self, version: T) {
        self.version = Some(version.to_string());
    }

    /// This method is used at the creation of a Jargon struct to add an app version.
    pub fn version<T: ToString>(mut self, version: T) -> Jargon {
        self.version = Some(version.to_string());
        self
    }

    /// This method is used at the creation of a Jargon struct to add an app argument.
    pub fn argument<T: Into<Key>>(mut self, arg: T) -> Jargon {
        let mut v: Vec<Key> = vec![arg.into(); 1];
        self.keys.0.append(&mut v);
        self
    }

    /// Add an argument to an already existing argument.
    pub fn add_argument<T: Into<Key>>(&mut self, arg: T) {
        let mut v: Vec<Key> = vec![arg.into(); 1];
        self.keys.0.append(&mut v);
    }

    /// Checks for a passed argument
    pub fn arg_bool<T: ToString>(&self, key: T) -> bool {
        let key: Key = match self.get_key(key) {
            Some(k) => k,
            None => return false,
        };

        let long = match key.get_long() {
            Some(l) => format!("-{}", l),
            None => return false,
        };

        let short = match key.get_short() {
            Some(s) => format!("-{}", s),
            None => return false,
        };

        let count: usize = self.args.len();

        for i in 0..count {
            if self.args[i] == short || self.args[i] == long {
                return true;
            }
        }
        false
    }

    /// Used internally to create a clone of a key to check the command line
    fn get_key<T: ToString>(&self, key: T) -> Option<Key> {
        let key: String = key.to_string();
        for k in self.keys.0.iter() {
            if k.name == key {
                return Some(k.to_owned());
            }
        }
        None
    }

    /// Argument to get a parameter from the command line. Resturns `Option<String>` when passed by
    /// the user, `None` otherwise.
    pub fn option_arg_str<T: ToString>(&self, key: T) -> Option<String> {
        let key: Key = self.get_key(key)?;

        let long: String = format!("--{}", key.get_long()?);
        let short: String = format!("-{}", key.get_short()?);

        let count: usize = self.args.len();
        let max: usize = count - 1;

        for i in 0..count {
            if (self.args[i] == short || self.args[i] == long) && i < max && !self.args[i + 1].starts_with('-')
            {
                return Some(self.args[i + 1].to_string());
            }
        }

        None
    }

    /// Finds and returns the String from an argument, returns Result<Error> on failure.
    pub fn arg_str<T: ToString>(&self, key: T) -> Result<String> {
        match self.option_arg_str(key.to_string()) {
            Some(s) => Ok(s),
            None => notfound!(key.to_string()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(missing_docs)]
struct Keys(Vec<Key>);

impl Keys {
    #[allow(missing_docs)]
    pub fn new() -> Keys {
        Keys(Vec::new())
    }
}

/// # Key
///
/// A Key is any argument on the command line, starting with `--` for long, `-` for short.
/// The key struct contains the name, short (optional), and long (optional) flag for every key.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Key {
    name: String,
    short: Option<char>,
    long: Option<String>,
}

impl Key {
    /// Creates an instance of the `Key` struct, requires a name.
    pub fn new<T: ToString>(name: T) -> Key {
        Key {
            name: name.to_string(),
            short: None,
            long: None,
        }
    }

    /// Returns the name of they key.
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// Returns the short (`-`) flag from the key, `Option<String>` if it exists, `None` if not.
    pub fn get_short(&self) -> Option<String> {
        let sh = self.short.to_owned()?;
        Some(sh.to_string())
    }

    /// Returns the long (`--`) flag from the key, `Option<String>` if it exists, `None` if not.
    pub fn get_long(&self) -> Option<String> {
        self.long.to_owned()
    }

    /// Used at the creation of a key to add a short (`-`) flag to the key.
    pub fn short<T: Into<Key>>(mut self, short: T) -> Key {
        let short: Key = short.into();
        self.short = short.short;
        self
    }

    /// Used at the creation of a key to add a longs (`--`) flag to the key.
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

        if let Some(pre) = s.strip_prefix('-') {
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

        Key { name, short, long }
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

            if let Some(pre) = s.strip_prefix('-') {
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

        Key { name, short, long }
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
        let app: crate::Jargon = crate::Jargon::new("app_add", args)
            .argument(crate::Key::new("add").short("-a").long("--add"));

        assert_eq!(
            crate::Jargon {
                name: "app_add".to_string(),
                author: None,
                version: None,
                keys: crate::Keys(
                    [crate::Key {
                        name: "add".to_string(),
                        short: Some('a'),
                        long: Some("add".to_string())
                    }; 1]
                        .to_vec()
                ),
                args: ["app_add".to_string(); 1].to_vec(),
            },
            app
        )
    }

    #[test]
    pub fn bool_arg_short_t() {
        let args: Vec<String> = ["bool_arg_short_t".to_string(), "-a".to_string()].to_vec();

        let app: crate::Jargon = crate::Jargon::new("bool_arg_short_t", args)
            .argument(crate::Key::new("add").short("-a").long("--add"));

        let b = app.arg_bool("add");

        assert!(b);
    }

    #[test]
    fn keys() {
        let key: crate::Key = crate::Key::new("key").short("-k").long("--key");

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
        let args: Vec<String> =
            ["arg_str".to_string(), "--add".to_string(), "1".to_string()].to_vec();

        let add: crate::Key = crate::Key::new("add").short("-a").long("--add");
        let app: crate::Jargon = crate::Jargon::new("arg_str", args).argument(add);

        assert_eq!(app.option_arg_str("add"), Some("1".to_string()));
    }

    #[test]
    fn arg_str_none() {
        let args: Vec<String> = ["arg_str".to_string(), "--add".to_string()].to_vec();

        let add: crate::Key = crate::Key::new("add").short("-a").long("--add");

        let app: crate::Jargon = crate::Jargon::new("arg_str", args).argument(add);

        assert_eq!(app.option_arg_str("add"), None);
    }

    #[test]
    fn arg_str_req() {
        let args: Vec<String> = ["arg_str_req".to_string(), "-a".to_string(), "34".to_string()].to_vec();

        let app: crate::Jargon = crate::Jargon::new("arg_str_req", args)
            .argument(
                crate::Key::new("age")
                    .short("-a")
                    .long("--age")
            );

        app.arg_str("age").unwrap();
    }

    #[test]
    #[should_panic]
    fn arg_str_req_f() {
        let args: Vec<String> = ["arg_str_req".to_string(), "-a".to_string()].to_vec();

        let app: crate::Jargon = crate::Jargon::new("arg_str_req", args)
            .argument(
                crate::Key::new("age")
                    .short("-a")
                    .long("--age")
            );

        app.arg_str("age").unwrap();
    }
}
