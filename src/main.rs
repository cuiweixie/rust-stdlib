mod iterator;

fn main() {
    let a = 1;
    match a {
        1 => println!("a is 1"),
        2 => println!("a is 2"),
        _ => println!("a is something else"),
    }
}
