extern crate backend;

use backend::names::RandonAnimalGenerator;
use backend::names::RandomNameGenerator;
use std::env;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
// use crate::names::read_random_line_in_file;
        // Returns a string from a random line in a file
    
    // Returns a string from a random line in a file



#[test]
fn test_create_random_animal_gives_name(){
    let random_firstname_generator = RandonAnimalGenerator{};
    let compound = random_firstname_generator.create_rand_name();

    assert!(compound.animal.len() > 1);
    assert!(compound.adjective.len() > 1);
}




