use super::Error;
use super::Key;
use std::result::Result;

/// # Jargon
///
/// This is the main struct in this crate. This is what is used to handle arguments,
/// and get arguments' values.
///
/// # Example
///
/// ```
/// use jargon_args::Jargon;
/// let mut jargon: Jargon = Jargon::from_env();
///
/// if jargon.contains(["-h", "--help"]) {
///     println!("help text");
///     std::process::exit(0);
/// }
///
/// // ...
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Jargon(pub(crate) Vec<String>);

impl Jargon {
    /// Extracts a program's arguments from the environment.
    pub fn from_env() -> Self {
        Self(std::env::args().collect())
    }

    /// Places provided vector into Jargon. Please have the program's name or subcommand's name at
    /// index `0`. 0 is always ignored.
    pub fn from_vec<T: ToString>(v: Vec<T>) -> Self {
        Self(v.iter().map(|x| x.to_string()).collect())
    }

    /// Checks if provided key is given in arguments. Removes it.
    pub fn contains<K: Into<Key>>(&mut self, key: K) -> bool {
        let key: Key = key.into();
        let len: usize = self.0.len();

        match key {
            Key::Dual {
                char: c,
                s_txt: s,
                l_txt: l,
            } => {
                let s: Key = Key::Short { char: c, txt: s };
                let l: Key = Key::Long { char: c, txt: l };
                for i in 0..len {
                    let cont: Key = self.0[i].clone().into();
                    if cont == s || cont == l {
                        self.0.remove(i);
                        return true;
                    }
                }
            }
            key => {
                for i in 0..len {
                    let cont: Key = self.0[i].clone().into();
                    if cont == key {
                        self.0.remove(i);
                        return true;
                    }
                }
            }
        }

        false
    }

    #[cfg(feature = "no_mut")]
    /// Checks if provided key is given in arguments. Dose not remove it.
    pub fn contains_nomut<K: Into<Key>>(&self, key: K) -> bool {
        let m = self.0.clone();
        let mut m = Self(m);
        m.contains(key)
    }

    /// Runs function that does not return a value if specified key exists.
    /// Removes the program's name from provided vector.
    pub fn on_subcommand<K: Into<Key>, F: FnMut(Vec<String>)>(&mut self, key: K, mut f: F) {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish());
            }
        }
    }

    /// Runs function that returns Option<T> if specified key exists.
    /// Removes the program's name from provided vector.
    pub fn opt_on_subcommand<K: Into<Key>, F: FnMut(Vec<String>) -> Option<T>, T>(
        &mut self,
        key: K,
        mut f: F,
    ) -> Option<T> {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish());
            }
        }
        None
    }

    /// Runs function that returns Result<T, jargon_args::Error> if specified key exists.
    /// Removes the program's name from provided vector.
    pub fn res_on_subcommand<K: Into<Key>, F: FnMut(Vec<String>) -> Result<T, Error>, T>(
        &mut self,
        key: K,
        mut f: F,
    ) -> Result<T, Error> {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish());
            }
        }

        Err(Error::MissingArg(key))
    }

    /// Checks if key exists, removes it, and returns it and all remaining arguments in
    /// Some(Vec<String>). None if key isn't in arguments.
    pub fn subcommand<K: Into<Key>>(&mut self, key: K) -> Option<Vec<String>> {
        let mut v: Vec<String> = Vec::new();
        self.on_subcommand(key, |vv| v = vv);
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }

    #[cfg(feature = "no_mut")]
    /// Checks if key exists, removes it without modifying your Jargon variable,
    /// and returns it and all remaining arguments in
    /// Some(Vec<String>). None if key isn't in arguments.
    pub fn subcommand_nomut<K: Into<Key>>(&self, key: K) -> Option<Vec<String>> {
        Jargon::from_vec(self.0.clone()).subcommand(key)
    }

    /// Checks for provided key in arguments, removes it, returns Some(String) with the value after it if there is one.
    /// None is there is no value.
    pub fn option_arg<K: Into<Key>>(&mut self, key: K) -> Option<String> {
        let key: Key = key.into();
        let len: usize = self.0.len();

        match key {
            Key::Dual {
                char: c,
                s_txt: s,
                l_txt: l,
            } => {
                let s: Key = Key::Short { char: c, txt: s };
                let l: Key = Key::Long { char: c, txt: l };
                for i in 0..len {
                    let cont: Key = self.0[i].clone().into();
                    if cont == s || cont == l {
                        if i >= self.0.len() - 1 {
                            return None;
                        }
                        return if !self.0[i + 1].starts_with(s.char())
                            || !self.0[i + 1].starts_with(l.char())
                        {
                            self.0.remove(i);
                            Some(self.0.remove(i))
                        } else {
                            None
                        };
                    }
                }
            }
            key => {
                for i in 0..len {
                    let cont: Key = self.0[i].clone().into();
                    if cont == key {
                        if i >= self.0.len() - 1 {
                            return None;
                        }
                        return if !self.0[i + 1].starts_with(key.char()) {
                            self.0.remove(i);
                            Some(self.0.remove(i))
                        } else {
                            None
                        };
                    }
                }
            }
        }

        None
    }

    /// Checks for provided key in arguments, removes it, returns Ok(String) with the value after it if there is one.
    /// Err(jargon_args::Error) is there is no value.
    pub fn result_arg<K: Into<Key>>(&mut self, key: K) -> Result<String, Error> {
        let key: Key = key.into();
        self.option_arg(key.clone()).ok_or(Error::MissingArg(key))
    }

    /// Drops your jargon instance and returns all remaining arguments.
    pub fn finish(self) -> Vec<String> {
        let mut v: Vec<String> = self.0;
        v.remove(0);
        v
    }
}
