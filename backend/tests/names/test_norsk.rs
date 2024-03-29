extern crate backend;

use backend::names::{common::{RandomGenderedNameGenerator, Gender}, norsk::RandomNorskGenerator};

fn get_all_female_names() -> Vec<String>{
    let animal : Vec<String> = include_str!("../../resources/norwegian_girls.csv").split("\n")
    .into_iter()
    .map(| e| e.replace("\r", ""))
    .map(| e | e.replace("\t", ""))
    .map(String::from)
    .collect(); 
    return animal;
}


fn get_all_male_names() -> Vec<String>{
    let animal : Vec<String> = include_str!("../../resources/norwegian_boys.csv").split("\n")
    .map(| e| e.replace("\r", ""))
    .map(| e | e.replace("\t", ""))
    .map(String::from)
    .collect(); 
    return animal;
}

fn get_all_last_names() -> Vec<String>{
    let animal : Vec<String> = include_str!("../../resources/norwegian_last.csv").split("\n")
    .map(| e| e.replace("\r", ""))
    .map(| e | e.replace("\t", ""))
    .map(String::from)
    .collect(); 
    return animal;
}

#[test]
fn test_verify_all_female_names_is_valid(){
    let names = get_all_female_names();

    assert!(!names.is_empty());
    assert!(names.len() > 400);
}

#[test]
fn test_verify_all_male_names_is_valid(){
    let names = get_all_male_names();

    assert!(!names.is_empty());
    assert!(names.len() > 400);
}

#[test]
fn test_verify_all_last_names_is_valid(){
    let names = get_all_last_names();

    assert!(!names.is_empty());
    assert!(names.len() > 3306);
}

#[test]
fn test_generate_female_name_is_valid(){
    let name = RandomNorskGenerator::generate_female_name().unwrap();
    let all_names : Vec<String> = get_all_female_names().into_iter().collect();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}

#[test]
fn test_generate_male_name_is_valid(){
    let name = RandomNorskGenerator::generate_male_name().unwrap();
    let all_names : Vec<String> = get_all_male_names().into_iter().collect();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}

#[test]
fn test_generate_last_name_is_valid(){
    let name = RandomNorskGenerator::generate_last_name().unwrap();
    let all_names : Vec<String> = get_all_last_names();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}

#[test]
fn test_generate_female_is_actual_female_name(){
    let name = RandomNorskGenerator::generate(Gender::FEMALE).unwrap();

    assert!(!name.first_name.is_empty());
    assert_eq!(name.gender, Gender::FEMALE);
    assert!(get_all_female_names().contains(&name.first_name));
    assert!(get_all_last_names().contains(&name.sur_name));
}

#[test]
fn test_generate_male_is_actual_male_name(){
    let name = RandomNorskGenerator::generate(Gender::MALE).unwrap();

    assert!(!name.first_name.is_empty());
    assert_eq!(name.gender, Gender::MALE);
    assert!(get_all_male_names().contains(&name.first_name));
    assert!(get_all_last_names().contains(&name.sur_name));
}

#[test]
fn test_generate_random_gender_returns_male_or_female(){
    let person = RandomNorskGenerator::generate(Gender::RANDOM).unwrap();

    assert!(person.gender == Gender::MALE || person.gender == Gender::FEMALE);
}