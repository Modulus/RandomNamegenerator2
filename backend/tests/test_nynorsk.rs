extern crate backend;

use backend::names::nynorsk::RandomNynorskGenerator;


#[test]
fn test_generate_female_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_female();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);
}
#[test]
fn test_generate_last_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_last_name();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);
}