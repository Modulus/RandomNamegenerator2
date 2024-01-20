extern crate backend;

use backend::names::{norse::RandomNorseGenerator, common::{RandomGenderedNameGenerator, Gender}};

fn get_all_norse_female_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norse_female.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}

fn get_all_norse_male_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norse_male.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}


fn get_all_norse_female_last_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norse_female_last.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}

fn get_all_norse_male_last_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norse_male_last.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}



#[test]
fn test_verify_get_all_norse_female_names(){
    let names = get_all_norse_female_names();

    assert_eq!(names.len(), 1053);
}

#[test]
fn test_verify_get_all_norse_female_last_names(){
    let names = get_all_norse_female_last_names();

    assert_eq!(names.len(), 75);
}

#[test]
fn test_verify_get_all_norse_male_names(){
    let names = get_all_norse_male_names();

    assert_eq!(names.len(), 120);
}

#[test]
fn test_verify_get_all_norse_male_last_names(){
    let names = get_all_norse_male_last_names();

    assert_eq!(names.len(), 75);
}


#[test]
fn test_verify_generate_female_name_is_contained_in_full_file(){
    let name = RandomNorseGenerator::generate_female().unwrap();

    assert!(!name.is_empty());

    let all_female_names : Vec<String> = get_all_norse_female_names().iter().map(|e|e.replace("\t", "")).collect();

    assert!(all_female_names.contains(&name));
}

#[test]
fn test_verify_generate_female_last_name_is_contained_in_full_file(){
    let name = RandomNorseGenerator::generate_female_last_name().unwrap();

    assert!(!name.is_empty());


    let all_names : Vec<String> = get_all_norse_female_last_names().iter().map(|e| e.replace("\t", "")).collect();

    assert!(all_names.contains(&name));
}

#[test]
fn test_verify_generate_male_name_is_contained_in_full_file(){
    let name = RandomNorseGenerator::generate_male().unwrap();

    assert!(!name.is_empty());


    let all_names = get_all_norse_male_names();

    assert!(all_names.contains(&name));
}

#[test]
fn test_verify_generate_male_last_name_is_contained_in_full_file(){
    let name = RandomNorseGenerator::generate_male_last_name().unwrap();

    assert!(!name.is_empty());


    let all_names : Vec<String> = get_all_norse_male_last_names().iter()
    .map(|e| e.replace("\t", "")).collect();

    assert!(all_names.contains(&name));
}

#[test]
fn test_verify_generate_male_is_valid(){
    let person = RandomNorseGenerator::generate(Gender::MALE);

    assert_eq!(person.gender, Gender::MALE);

    assert!(!person.first_name.is_empty());

    assert!(get_all_norse_male_names().contains(&person.first_name));
    assert!(get_all_norse_male_last_names().contains(&person.sur_name));
    
}


#[test]
fn test_verify_generate_female_is_valid(){
    let person = RandomNorseGenerator::generate(Gender::FEMALE);

    assert_eq!(person.gender, Gender::FEMALE);

    assert!(!person.first_name.is_empty());

    assert!(get_all_norse_female_names().iter().map(|e| e.replace("\t", "")).collect::<Vec<String>>().contains(&person.first_name));
    assert!(get_all_norse_female_last_names().contains(&person.sur_name));
}