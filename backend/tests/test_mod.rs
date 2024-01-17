extern crate backend;

use backend::names::animal::RandomAnimalGenerator;
use backend::names::common::RandomNameGenerator;


#[test]
fn test_create_random_animal_gives_name(){
    let compound = RandomAnimalGenerator::create_rand_name();

    assert!(compound.animal.len() > 1);
    assert!(compound.adjective.len() > 1);
}




