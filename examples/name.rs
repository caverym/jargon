use jargon::*;

fn main() {
    let mut j: Jargon = Jargon::from_env();
    let name_key: Key = "--name".into();

    match j.option_arg(name_key.clone()) {
        Some(n) => println!("Your name: {}", n),
        None => eprintln!("Missing argument: {}", name_key),
    }
}