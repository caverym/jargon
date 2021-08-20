use super::Key;
use super::Error;
use std::result::Result;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Jargon(pub(crate) Vec<String>);

impl Jargon {
    pub fn from_env() -> Self {
        Self(std::env::args().collect())
    }

    pub fn from_vec<T: ToString>(v: Vec<T>) -> Self {
        Self(v.iter().map(|x| x.to_string()).collect())
    }

    pub fn contains<K: Into<Key>>(&mut self, key: K) -> bool {
        let key: Key = key.into();
        let len: usize = self.0.len();

        match key {
            Key::Dual {
                char: c,
                s_txt: s,
                l_txt: l,
            } => {
                let s: Key = Key::Short {
                    char: c.clone(),
                    txt: s,
                };
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

    pub fn contains_unmut<K: Into<Key>>(&self, key: K) -> bool {
        let m = self.0.clone();
        let mut m = Self(m);
        m.contains(key)
    }

    pub fn on_subcommand<K: Into<Key>, F: Fn(Vec<String>)>(&mut self, key: K, f: F) {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish())
            }
        }
    }


    pub fn opt_on_subcommand<K: Into<Key>, F: Fn(Vec<String>) -> Option<T>, T>(&mut self, key: K, f: F) -> Option<T> {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish())
            }
        }
        None
    }

    pub fn ret_on_subcommand<K: Into<Key>, F: Fn(Vec<String>) -> Result<T, Error>, T>(&mut self, key: K, f: F) -> Result<T, Error> {
        let key: Key = key.into();
        for i in 0..self.0.len() {
            let cont: Key = self.0[i].clone().into();
            if cont.is_sub() && cont == key {
                return f(self.clone().finish())
            }
        }
        Err(Error::MissingArg(key.into()))
    }

    pub fn option_arg<K: Into<Key>>(&mut self, key: K) -> Option<String> {
        let key: Key = key.into();
        let len: usize = self.0.len();

        match key {
            Key::Dual {
                char: c,
                s_txt: s,
                l_txt: l,
            } => {
                let s: Key = Key::Short {
                    char: c.clone(),
                    txt: s,
                };
                let l: Key = Key::Long { char: c, txt: l };
                for i in 0..len {
                    let cont: Key = self.0[i].clone().into();
                    if cont == s || cont == l {
                        if i >= self.0.len()-1 {
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
                        if i >= self.0.len()-1 {
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

    pub fn result_arg<K: Into<Key>>(&mut self, key: K) -> Result<String, Error> {
        let key: Key = key.into();
        self.option_arg(key.clone()).ok_or(Error::MissingArg(key))
    }

    pub fn finish(self) -> Vec<String> {
        let mut v: Vec<String> = self.0;
        v.remove(0);
        v
    }
}
