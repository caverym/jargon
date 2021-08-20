use jargon::Jargon;

struct Args {
    multiple: bool,
    suffix: Option<String>,
    zero: bool,
    names: Vec<String>,
}

fn main() {
    let mut j: Jargon = Jargon::from_env();

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
