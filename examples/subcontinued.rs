use jargon_args::Jargon;

fn main() {
    let mut j: Jargon = Jargon::from_env();
    j.on_subcommand("go", go);
}

fn go(v: Vec<String>) {
    println!("go!");
    let mut j: Jargon = Jargon::from_vec(v);
    j.on_subcommand("goo", goo)
}

fn goo(v: Vec<String>) {
    println!("goo!");
    let mut j: Jargon = Jargon::from_vec(v);
    j.on_subcommand("gooo", gooo)
}

fn gooo(_: Vec<String>) {
    println!("gooo!");
}
