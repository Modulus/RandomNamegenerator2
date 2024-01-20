extern crate backend;

use backend::names::norse::RandomNorseGenerator;

fn get_all_norse_female_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../resources/norse_female.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}

fn get_all_norse_male_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../resources/norse_male.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}


fn get_all_norse_female_last_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../resources/norse_female_last.csv").split("\n").collect(); 
    return animal.into_iter().map(String::from).collect();
}

fn get_all_norse_male_last_names() -> Vec<String>{
    let animal : Vec<&str> = include_str!("../resources/norse_male_last.csv").split("\n").collect(); 
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
