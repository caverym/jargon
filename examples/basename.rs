use jargon_args::Jargon;

// Not required, but a helper struct to contain arguments and their data.
struct Args {
    multiple: bool,
    suffix: Option<String>,
    zero: bool,
    names: Vec<String>,
}

fn main() {
    let mut j: Jargon = Jargon::from_env(); // Get an instance of Jargon using `std::env::args()`

    if j.contains(["-h", "--help"]) { // check for help argument
        print!("{}", HELP);
        return;
    }

    if j.contains(["-v", "--version"]) { // check for version argument
        println!(
            "basename example for Jargon crate {}",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }

    let args = Args { // fill helper struct
        multiple: j.contains(["-a", "--multiple"]), // multiple
        suffix: j.option_arg(["-s", "--suffix"]), // suffix to remove
        zero: j.contains(["-z", "--zero"]), // terminate lines with null
        names: j.finish(), // get names
    };

    if args.names.is_empty() { // check if there are names
        println!("Missing NAMES");
        return;
    }

    let mut v: Vec<String> = vec![print(&args, &args.names[0])]; // initiate vector of names

    if args.multiple { // fill the rest if `-a` or `--multiple` was passed
        args.names.iter().skip(1).for_each(|name| v.append(&mut vec![print(&args, name)]));
    }

    if args.zero { // terminate with null if `-z` or `--zero` was passed
        v.iter().for_each(|name| print!("{}", name));
    } else { // terminate each name with space or new line
        v.iter().for_each(|name| println!("{} ", name));
    }
}

// extract basename and remove suffix
fn print(args: &Args, name: &String) -> String {
    if let Some(name) = name.split('/').last() {
        if let Some(suffix) = &args.suffix {
            if name.ends_with(suffix) {
                if let Some(n) = name.strip_suffix(suffix) {
                    return n.to_string();
                }
            }
        }
        return name.to_string();
    }
    name.to_string()
}

const HELP: &str = "Usage: basename NAME [SUFFIX]
  or:  basename OPTION... NAME...
Print NAME with any leading directory components removed.
If specified, also remove a trailing SUFFIX.

  -a, --multiple       support multiple arguments and treat each as a NAME
  -s, --suffix SUFFIX  remove a trailing SUFFIX; implies -a
  -z, --zero           end each output line with NUL, not newline
  -h, --help     display this help and exit
  -v, --version  output version information and exit
";
