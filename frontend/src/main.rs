extern crate backend;

use backend::{Generator, names::common::Gender};

fn main() {
    let name = Generator::generate();
    println!("Generating name!");
    println!("{:?}", name);

    let name2 = Generator::generate_nynorsk(Gender::FEMALE);

    println!("{:?}", name2);

    let name3 = Generator::generate_nynorsk(Gender::MALE);
    println!("{:?}", name3);

    let norse1 = Generator::generate_norse(Gender::MALE);
    println!("{:?}", norse1);


    let norse2 = Generator::generate_norse(Gender::FEMALE);
    println!("{:?}", norse2);

    let norsk1 = Generator::generate_norse(Gender::FEMALE);
    println!("{:?}", norsk1);
    let norsk2 = Generator::generate_norse(Gender::MALE);
    println!("{:?}", norsk2);

}
