extern crate backend;

use backend::names::{norse::RandomNorseGenerator, common::{RandomGenderedNameGenerator, Gender}};

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