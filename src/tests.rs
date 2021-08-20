use crate::{Jargon, Key, Error};

#[test]
fn short_key() {
    let key: Key = "-a".into();

    assert_eq!(
        key,
        Key::Short {
            char: '-',
            txt: 'a'
        }
    )
}

#[test]
fn long_key() {
    let key: Key = "--all".into();

    assert_eq!(
        key,
        Key::Long {
            char: '-',
            txt: "all".to_string()
        }
    )
}

#[test]
fn sub_key() {
    let key: Key = "beans".into();

    assert_eq!(
        key,
        Key::Sub {
            txt: "beans".into(),
        }
    )
}

#[test]
fn dual_key_be() {
    let dk: Key = ["-a", "--all"].into();

    assert_eq!(
        dk,
        Key::Dual {
            char: '-',
            s_txt: 'a',
            l_txt: "all".to_string(),
        }
    )
}

#[test]
fn dual_key_le() {
    let dk: Key = ["--all", "-a"].into();

    assert_eq!(
        dk,
        Key::Dual {
            char: '-',
            s_txt: 'a',
            l_txt: "all".to_string(),
        }
    )
}

#[test]
#[should_panic]
fn dual_key_with_sub() {
    let _: Key = ["-all", "beans"].into();
}

#[test]
fn fmt_short_key() {
    let key: Key = "-a".into();
    assert_eq!(format!("{}", key), "-a".to_string(),)
}

#[test]
fn fmt_long_key() {
    let key: Key = "--all".into();
    assert_eq!(format!("{}", key), "--all".to_string(),)
}

#[test]
fn fmt_dual_key() {
    let key: Key = ["-a", "--all"].into();
    assert_eq!(format!("{}", key), "-a, --all".to_string(),)
}

#[test]
fn fmt_sub_key() {
    let key: Key = "beans".into();
    assert_eq!(format!("{}", key), "beans".to_string())
}

#[test]
fn jargon() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert_eq!(
        j,
        Jargon {
            0: [
                "jargon".to_string(),
                "-a".to_string(),
                "-b".to_string(),
                "beans".to_string(),
            ]
                .to_vec(),
        }
    )
}

#[test]
fn jargon_finish() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    j.contains("-a");

    assert_eq!(
        j.finish(),
        vec!["-b".to_string(), "beans".to_string(),]
    )
}

#[test]
fn jargon_contains_short_t() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains_unmut("-a"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-a".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_short_f() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains_unmut("-a"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_short_t_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains("-a"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_short_f_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains("-a"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_long_t() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "--all".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains_unmut("--all"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "--all".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_long_f() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains_unmut("--all"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_long_t_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "--all".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains("--all"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_long_f_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains("--all"));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_s_t() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains_unmut(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-a".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_s_f() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains_unmut(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_s_t_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-a".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_l_f_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_l_t() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "--all".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains_unmut(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "--all".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_l_f() {
    let j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(!j.contains_unmut(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_contains_dual_l_t_rmv() {
    let mut j = Jargon::from_vec(vec![
        "jargon".to_string(),
        "--all".to_string(),
        "-b".to_string(),
        "beans".to_string(),
    ]);

    assert!(j.contains(["-a", "--all"]));

    assert_eq!(
        j,
        Jargon(vec![
            "jargon".to_string(),
            "-b".to_string(),
            "beans".to_string(),
        ])
    )
}

#[test]
fn jargon_arg_option_short_t() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("-a"), Some("hello".to_string()),)
}

#[test]
fn jargon_arg_option_short_fa() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
        "--hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("-a"), None,)
}

#[test]
fn jargon_arg_option_short_fm() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
    ]);

    assert_eq!(j.option_arg("-a"), None,)
}

#[test]
fn jargon_arg_option_short_f() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("-a"), None,)
}

#[test]
fn jargon_arg_option_long_t() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("--all"), Some("hello".to_string()),)
}

#[test]
fn jargon_arg_option_long_fa() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
        "--hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("--all"), None,)
}

#[test]
fn jargon_arg_option_long_fm() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
    ]);

    assert_eq!(j.option_arg("--all"), None,)
}

#[test]
fn jargon_arg_option_long_f() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg("--all"), None,)
}

#[test]
fn jargon_arg_option_dual_s_t() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), Some("hello".to_string()),)
}

#[test]
fn jargon_arg_option_dual_s_fa() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
        "--hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn jargon_arg_option_dual_s_fm() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "-a".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn jargon_arg_option_dual_s_f() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn jargon_arg_option_dual_l_t() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), Some("hello".to_string()),)
}

#[test]
fn jargon_arg_option_dual_l_fa() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
        "--hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn jargon_arg_option_dual_l_fm() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "--all".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn jargon_arg_option_dual_l_f() {
    let mut j = Jargon::from_vec(vec![
        "jargon_arg_option_short_t".to_string(),
        "hello".to_string(),
        "world".to_string(),
    ]);

    assert_eq!(j.option_arg(["-a", "--all"]), None,)
}

#[test]
fn key_short_is_short() {
    let k: Key = "-a".into();
    assert!(k.is_short())
}

#[test]
fn key_short_is_long() {
    let k: Key = "-a".into();
    assert!(!k.is_long())
}

#[test]
fn key_short_is_dual() {
    let k: Key = "-a".into();
    assert!(!k.is_dual())
}

#[test]
fn key_short_is_sub() {
    let k: Key = "-a".into();
    assert!(!k.is_sub())
}

#[test]
fn key_long_is_short() {
    let k: Key = "--all".into();
    assert!(!k.is_short())
}

#[test]
fn key_long_is_long() {
    let k: Key = "--all".into();
    assert!(k.is_long())
}

#[test]
fn key_long_is_dual() {
    let k: Key = "--all".into();
    assert!(!k.is_dual())
}

#[test]
fn key_long_is_sub() {
    let k: Key = "--all".into();
    assert!(!k.is_sub())
}

#[test]
fn key_dual_is_short() {
    let k: Key = ["-a", "--all"].into();
    assert!(!k.is_short())
}

#[test]
fn key_dual_is_long() {
    let k: Key = ["-a", "--all"].into();
    assert!(!k.is_long())
}

#[test]
fn key_dual_is_dual() {
    let k: Key = ["-a", "--all"].into();
    assert!(k.is_dual())
}

#[test]
fn key_dual_is_sub() {
    let k: Key = ["-a", "--all"].into();
    assert!(!k.is_sub())
}

#[test]
fn key_sub_is_short() {
    let k: Key = "subcmd".into();
    assert!(!k.is_short())
}

#[test]
fn key_sub_is_long() {
    let k: Key = "subcmd".into();
    assert!(!k.is_long())
}

#[test]
fn key_sub_is_dual() {
    let k: Key = "subcmd".into();
    assert!(!k.is_dual())
}

#[test]
fn key_sub_is_sub() {
    let k: Key = "subcmd".into();
    assert!(k.is_sub())
}

#[test]
fn key_short_char() {
    let k: Key = "-a".into();
    assert_eq!(k.char(), '-',)
}

#[test]
fn key_long_char() {
    let k: Key = "--all".into();
    assert_eq!(k.char(), '-')
}

#[test]
fn key_dual_char() {
    let k: Key = ["-a", "--all"].into();
    assert_eq!(k.char(), '-')
}

#[test]
fn key_sub_char() {
    let k: Key = "beans".into();
    assert_eq!(k.char(), '\0',)
}

#[test]
fn key_short_text() {
    let k: Key = "-a".into();
    assert_eq!(k.text(), "a".to_string(),)
}

#[test]
fn key_long_text() {
    let k: Key = "--all".into();
    assert_eq!(k.text(), "all".to_string(),)
}

#[test]
fn key_dual_text() {
    let k: Key = ["-a", "--all"].into();
    assert_eq!(k.text(), "all",)
}

#[test]
fn key_sub_text() {
    let k: Key = "beans".into();
    assert_eq!(k.text(), "beans",)
}

#[test]
fn from_env() {
    let j = Jargon::from_env();
    assert!(!j.0.is_empty());
}

#[test]
fn on_sub_e() {
    let mut j = Jargon::from_vec(
        vec![
            "on_sub".to_string(),
            "subsub".to_string(),
        ]
    );

    fn subsub(v: Vec<String>) {
        assert_eq!(
            v,
            vec![
                "subsub".to_string(),
            ]
        )
    }

    j.on_subcommand("subsub", subsub);
}

#[test]
fn on_sub_w() {
    let mut j = Jargon::from_vec(
        vec![
            "on_sub".to_string(),
            "subsub".to_string(),
            "test".to_string(),
        ]
    );

    fn subsub(v: Vec<String>) {
        assert_eq!(
            v,
            vec![
                "subsub".to_string(),
                "test".to_string(),
            ]
        )
    }

    j.on_subcommand("subsub", subsub);
}

#[test]
fn fmt_error_missing() {
    assert_eq!(
        Error::MissingArg("--all".into()).to_string(),
        "Missing argument: '--all'".to_string(),
    )
}

#[test]
fn fmt_error_other() {
    let e = Error::Other(std::io::Error::new(std::io::ErrorKind::AddrInUse, "test").to_string());
    assert_eq!(
        e.to_string(),
        "test".to_string(),
    )
}
