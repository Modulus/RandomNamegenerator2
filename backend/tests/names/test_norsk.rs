extern crate backend;

use backend::names::{common::{RandomGenderedNameGenerator, Gender}, norsk::RandomNorskGenerator};

fn get_all_female_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norwegian_girls.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}


fn get_all_male_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norwegian_boys.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}

fn get_all_last_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../../resources/norwegian_last.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
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
    let name = RandomNorskGenerator::generate_female_name();
    let all_names : Vec<String> = get_all_female_names().into_iter().collect();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}

#[test]
fn test_generate_male_name_is_valid(){
    let name = RandomNorskGenerator::generate_male_name();
    let all_names : Vec<String> = get_all_male_names().into_iter().collect();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}

#[test]
fn test_generate_last_name_is_valid(){
    let name = RandomNorskGenerator::generate_last_name();
    let all_names : Vec<String> = get_all_last_names().into_iter().collect();

    assert!(!name.is_empty());
    assert!(all_names.contains(&name));
}