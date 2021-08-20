use jargon::Jargon;

fn main() {
    let mut j: Jargon = Jargon::from_env();

    j.on_subcommand("list", list);
    j.on_subcommand("backwards", list_backwards);
}

fn list(v: Vec<String>) {
    v.iter().skip(1).for_each(|x| println!("{}", x))
}

fn list_backwards(v: Vec<String>) {
    v.iter().skip(1).rev().for_each(|x| println!("{}", x))
}
