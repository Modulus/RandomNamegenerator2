extern crate backend;

use backend::Generator;

fn main() {
    let name = Generator::generate();
    println!("Generating name!");
    println!("{:?}", name);
}
