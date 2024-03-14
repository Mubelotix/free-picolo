mod data;
pub use data::*;

fn main() {
    let items = parse_items();
    for item in items {
        println!("{:?}", item);
    }

    println!("Hello, world!");
}
