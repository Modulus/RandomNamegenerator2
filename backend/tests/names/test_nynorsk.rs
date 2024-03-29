extern crate backend;

use std::collections::HashSet;

use backend::names::{nynorsk::RandomNynorskGenerator, common::{RandomGenderedNameGenerator, Gender}};

fn get_all_female_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/first_female_a.csv")
    .split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/first_female_b.csv")
    .split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part"))
    .collect(); 

    return combine_vectors(part1, part2);
}

fn combine_vectors(list_a: Vec<&str>, list_b: Vec<&str>) -> Vec<String> {
    let mut all : Vec<String> = Vec::new();

    for outer in list_a {
        for inner in list_b.clone() {
            println!("{}{}", outer, inner);
            let mut compound = String::from(outer);
            compound.push_str(inner);
            all.push(compound);        
        }
    }

    return all.into_iter().map(| e | e.replace("\r", "")).map(| e | e.replace("\t", "")).collect();
}

fn get_all_male_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/first_male_a.csv").split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/first_male_b.csv").split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part")).collect();

    return combine_vectors(part1, part2);
}

fn get_all_last_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/last_a.csv").split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/last_b.csv").split("\n").filter(|e| !e.contains("name")).filter(|e| !e.contains("part")).collect();

    return combine_vectors(part1, part2);
}

#[test]
fn test_verify_get_all_female_names_is_valid(){
    let names = get_all_female_names();

    assert!(!names.is_empty());
    assert!(names.len() >= 132);

    let unique = names.clone().into_iter().collect::<HashSet<String>>();
    assert_eq!(unique.len(), names.len());    
}


#[test]
fn test_verify_get_all_male_names_is_valid(){
    let names = get_all_male_names();

    assert!(!names.is_empty());
    assert!(names.len() >= 261);

    let unique = names.clone().into_iter().collect::<HashSet<String>>();
    assert_eq!(unique.len(), names.len());    
}

#[test]
fn test_verify_get_all_last_names_is_valid(){
    let names = get_all_last_names();

    assert!(!names.is_empty());
    assert!(names.len() >= 922);

    let unique = names.clone().into_iter().collect::<HashSet<String>>();
    assert_eq!(unique.len(), names.len());

}

#[test]
fn test_generate_female_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_female().unwrap();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);

    assert!(get_all_female_names().contains(&name));
}


#[test]
fn test_generate_male_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_male().unwrap();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);

    assert!(get_all_male_names().contains(&name));
}

#[test]
fn test_generate_last_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_last_name().unwrap();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);

    assert!(get_all_last_names().contains(&name));
}

#[test]
fn test_generate_given_female_generates_actual_female_name(){
    let person = RandomNynorskGenerator::generate(Gender::FEMALE).unwrap();

    let all_first_names = get_all_female_names();
    let all_last_names = get_all_last_names();

    assert!(!person.first_name.is_empty());
    assert!(!person.sur_name.is_empty());

    assert_eq!(Gender::FEMALE, person.gender);
    assert!(all_first_names.contains(&person.first_name));
    assert!(all_last_names.contains(&person.sur_name));
}


#[test]
fn test_generate_given_male_generates_actual_male_name(){
    let person = RandomNynorskGenerator::generate(Gender::MALE).unwrap();

    let all_first_names = get_all_male_names();
    let all_last_names = get_all_last_names();

    assert!(!person.first_name.is_empty());
    assert!(!person.sur_name.is_empty());

    assert_eq!(Gender::MALE, person.gender);
    assert!(all_first_names.contains(&person.first_name));
    assert!(all_last_names.contains(&person.sur_name));
}


#[test]
fn test_generate_random_person_is_either_female_or_male(){
    let person = RandomNynorskGenerator::generate(Gender::RANDOM).unwrap();

    assert!(person.gender == Gender::FEMALE || person.gender == Gender::MALE);
}