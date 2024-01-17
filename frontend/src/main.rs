extern crate backend;

use backend::Generator;

fn main() {
    let generator = Generator::new();
    let name = generator.generate();
    println!("Generating name!");
    println!("{:?}", name);
}
