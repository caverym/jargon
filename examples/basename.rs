use jargon::Jargon;

struct Args {
    multiple: bool,
    suffix: Option<String>,
    zero: bool,
    names: Vec<String>,
}

fn main() {
    let mut j: Jargon = Jargon::from_env();

    if j.contains(["-h", "--help"]) {
        print!(
            "Usage: basename NAME [SUFFIX]
  or:  basename OPTION... NAME...
Print NAME with any leading directory components removed.
If specified, also remove a trailing SUFFIX.

  -a, --multiple       support multiple arguments and treat each as a NAME
  -s, --suffix SUFFIX  remove a trailing SUFFIX; implies -a
  -z, --zero           end each output line with NUL, not newline
      --help     display this help and exit
      --version  output version information and exit
"
        );
        return;
    }

    if j.contains(["-v", "--version"]) {
        println!("basename example for Jargon crate {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let args = Args {
        multiple: j.contains(["-a", "--multiple"]),
        suffix: j.option_arg(["-s", "--suffix"]),
        zero: j.contains("--zero"),
        names: j.finish(),
    };

    if !args.multiple {
        print(&args, &args.names[0])
    } else {
        args.names.iter().for_each(|name| print(&args, name));
    }

    if !args.zero {
        println!()
    }
}

fn print(args: &Args, name: &String) {
    if let Some(name) = name.split('/').last() {
        if let Some(suffix) = &args.suffix {
            if name.ends_with(suffix) {
                if let Some(n) = name.strip_suffix(suffix) {
                    if args.zero {
                        print!("{}", n)
                    } else {
                        print!("{} ", n)
                    }
                } else {
                    if args.zero {
                        print!("{}", name)
                    } else {
                        print!("{} ", name)
                    }
                }
            } else {
                if args.zero {
                    print!("{}", name)
                } else {
                    print!("{} ", name)
                }
            }
        } else {
            if args.zero {
                print!("{}", name)
            } else {
                print!("{} ", name)
            }
        }
    } else {
        if args.zero {
            print!("{}", name)
        } else {
            print!("{} ", name)
        }
    }
}
