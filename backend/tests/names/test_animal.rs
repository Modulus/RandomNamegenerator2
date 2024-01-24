extern crate backend;

use std::collections::HashSet;

use backend::names::animal::RandomAnimalGenerator;
use backend::names::common::RandomNameGenerator;

fn get_all_animal_names() -> Vec<String>{
    let animal : Vec<String> = include_str!("../../resources/animals.csv").split("\n")
    .map(|e| e.replace("\r", ""))
    .filter(|e| !e.contains("name"))
    .collect(); 
    return animal;
}

fn get_all_adjectives() -> Vec<String> {
    let adjectives : Vec<String> = include_str!("../../resources/adjectives.csv").split("\n")
    .map(|e| e.replace("\r", ""))
    .filter(|e| !e.contains("adjective"))
    .map(String::from).collect();

    return adjectives;
}

#[test]
fn test_verify_length_of_names(){
    let animals = get_all_animal_names();
    let expected_length = 1032;

    assert_eq!(animals.len(), expected_length);

    let unique = animals.into_iter().collect::<HashSet<String>>();

    assert_eq!(unique.len(), expected_length);
}

#[test]
fn test_verify_length_of_adjectives(){
    let adjectives = get_all_adjectives();

    let expected_length = 292;

    assert_eq!(adjectives.len(), expected_length);

    let unique = adjectives.into_iter().collect::<HashSet<String>>();

    //There seems to be a duplicate here
    assert_eq!(unique.len(), 291);

}

#[test]
fn test_verify_generated_animal_name_is_included_in_file(){
    let animals = get_all_animal_names();
    let generated_animal = RandomAnimalGenerator::generate();
    println!("ALL {:?}", &animals);
    println!("Generated {:?}", &generated_animal);

    assert!(animals.contains(&generated_animal.animal));
}

#[test]
fn test_verify_generated_adjective_is_included_in_file(){
    let adjectives : Vec<String> = get_all_adjectives().iter().map(|e| e.replace("\t", "")).collect();
    let generated_animal = RandomAnimalGenerator::generate();

    assert!(adjectives.contains(&generated_animal.adjective));
}


#[test]
fn test_create_random_animal_gives_name(){
    let compound = RandomAnimalGenerator::generate();

    assert!(compound.animal.len() > 1);
    assert!(compound.adjective.len() > 1);
}

#[test]
fn test_get_animal_names_does_not_contain_any_slash_r(){
    let animals = get_all_animal_names();
    let r = String::from("\r");
    println!("ANIMALS: {:?}", animals);
    assert!(animals.len() > 0);
    assert!(animals.contains(&r) == false);
}




