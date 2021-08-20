use std::fmt::Formatter;

/// # Key
///
/// This is the Key enum that represents processable arguments. This has four variants.
///
/// ## Dual
///
/// Key::Dual represents ONLY Key::Long and Key::Short in one, they both must start with the same character.
///
/// ```
/// let key: jargon_args::Key = ["-a", "--all"].into();
/// assert!(key.is_dual())
/// ```
///
/// ## Long
///
/// Key::Long represents a full name argument like `--all`.
///
/// ```
/// let key: jargon_args::Key = "--all".into();
/// assert!(key.is_long())
/// ```
///
/// ## Short
///
/// Key::Short represents a single letter argument like `-a`.
///
/// ```
/// let key: jargon_args::Key = "-a".into();
/// assert!(key.is_short())
/// ```
///
/// ## sub
///
/// Key::Sub represents a subcommand argument, anything not converted into any other time becomes Key::Sub.
///
/// ```
/// let key: jargon_args::Key = "list".into();
/// assert!(key.is_sub())
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Key {
    /// # Dual
    ///
    /// Key::Dual represents ONLY Key::Long and Key::Short in one, they both must start with the same character.
    ///
    /// ```
    /// let key: jargon_args::Key = ["-a", "--all"].into();
    /// assert!(key.is_dual())
    /// ```
    Dual {
        /// The character at the beginning of each argument.
        char: char,
        /// The single character for the short argument.
        s_txt: char,
        /// The word for the long argument.
        l_txt: String,
    },

    /// # Long
    ///
    /// Key::Long represents a full name argument like `--all`.
    ///
    /// ```
    /// let key: jargon_args::Key = "--all".into();
    /// assert!(key.is_long())
    /// ```
    Long {
        /// The character at the beginning of the argument.
        char: char,
        /// The word for the argument.
        txt: String,
    },

    /// # Short
    ///
    /// Key::Short represents a single letter argument like `-a`.
    ///
    /// ```
    /// let key: jargon_args::Key = "-a".into();
    /// assert!(key.is_short())
    /// ```
    Short {
        /// The character at the beginning of the argument.
        char: char,
        /// The character for the argument.
        txt: char,
    },

    /// # sub
    ///
    /// Key::Sub represents a subcommand argument, anything not converted into any other time becomes Key::Sub.
    ///
    /// ```
    /// let key: jargon_args::Key = "list".into();
    /// assert!(key.is_sub())
    /// ```
    Sub {
        /// The word for the subcommand.
        txt: String,
    },
}

impl Key {
    /// Return the char at the beginning of each argument, Key::sub returns `\0`.
    pub fn char(&self) -> char {
        match self {
            Key::Dual { char: c, .. } => *c,
            Key::Long { char: c, .. } => *c,
            Key::Short { char: c, .. } => *c,
            Key::Sub { .. } => '\0',
        }
    }

    /// Returns only the text of each argument as String. `--all` is `all`.
    pub fn text(&self) -> String {
        match self {
            Key::Dual { l_txt: txt, .. } => txt.clone(),
            Key::Long { txt, .. } => txt.clone(),
            Key::Short { txt, .. } => txt.to_string(),
            Key::Sub { txt, .. } => txt.clone(),
        }
    }

    /// Returns true if Key is Key::Dual.
    pub fn is_dual(&self) -> bool {
        match self {
            Key::Dual { .. } => true,
            Key::Long { .. } => false,
            Key::Short { .. } => false,
            Key::Sub { .. } => false,
        }
    }

    /// Returns true if Key is Key::Long.
    pub fn is_long(&self) -> bool {
        match self {
            Key::Dual { .. } => false,
            Key::Long { .. } => true,
            Key::Short { .. } => false,
            Key::Sub { .. } => false,
        }
    }

    /// Returns true if Key is Key::Short.
    pub fn is_short(&self) -> bool {
        match self {
            Key::Dual { .. } => false,
            Key::Long { .. } => false,
            Key::Short { .. } => true,
            Key::Sub { .. } => false,
        }
    }

    /// Returns true if Key is Key::Sub.
    pub fn is_sub(&self) -> bool {
        match self {
            Key::Dual { .. } => false,
            Key::Long { .. } => false,
            Key::Short { .. } => false,
            Key::Sub { .. } => true,
        }
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        let chars: Vec<char> = s.chars().collect();

        if !chars[0].is_alphabetic() {
            let char: char = chars[0];
            if s.len() == 2 {
                let txt: char = chars[1];

                Self::Short { char, txt }
            } else {
                let mut txt: String = String::new();

                chars
                    .iter()
                    .skip_while(|x| **x == chars[0])
                    .for_each(|x| txt = format!("{}{}", txt, x));

                Self::Long { char, txt }
            }
        } else {
            Self::Sub { txt: s }
        }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl<T: Clone + Into<Key>> From<[T; 2]> for Key {
    fn from(dk: [T; 2]) -> Self {
        let one: Key = dk[0].clone().into();
        let two: Key = dk[1].clone().into();

        if one.is_sub() || two.is_sub() {
            panic!("dual cannot contain a subcommand!");
        }

        if one.is_long() && two.is_short() {
            Key::Dual {
                char: one.char(),
                s_txt: two.text().parse().unwrap(),
                l_txt: one.text(),
            }
        } else {
            Key::Dual {
                char: two.char(),
                s_txt: one.text().parse().unwrap(),
                l_txt: two.text(),
            }
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Dual {
                    char: c,
                    s_txt: s,
                    l_txt: l,
                } => format!("{}{}, {}{}{}", c, s, c, c, l),
                Key::Long { char: c, txt: t } => format!("{}{}{}", c, c, t),
                Key::Short { char: c, txt: t } => format!("{}{}", c, t),
                Key::Sub { txt: t } => t.to_string(),
            }
        )
    }
}
