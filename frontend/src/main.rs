extern crate backend;

use backend::{Generator, names::common::Gender};

fn main() {
    let name = Generator::generate();
    println!("Generating name!");
    println!("{:?}", name);

    let name2 = Generator::generate_nynorsk(Gender::FEMALE);

    println!("{:?}", name2);

    let name3 = Generator::generate_nynorsk(Gender::MALE);
    println!("{:?}", name3)
}
