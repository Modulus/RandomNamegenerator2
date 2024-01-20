extern crate backend;

use backend::names::nynorsk::RandomNynorskGenerator;

fn get_all_female_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/first_female_a.csv")
    .split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/first_female_b.csv")
    .split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect(); 

    return combine_vectors(part1, part2);
}

fn combine_vectors(list_a: Vec<&str>, list_b: Vec<&str>) -> Vec<String> {
    let mut all : Vec<String> = Vec::new();

    for outer in list_a {
        for inner in list_b.clone() {
            println!("{}{}", outer, inner);
            let compound = format!("{}{}", String::from(outer), String::from(inner));
            all.push(compound);        
        }
    }

    return all;
}

fn get_all_male_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/first_male_a.csv").split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/first_male_b.csv").split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect();

    return combine_vectors(part1, part2);
}

fn get_all_last_names() -> Vec<String>{
    let part1 : Vec<&str> = include_str!("../../resources/nynorsk/last_a.csv").split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect();
    let part2 : Vec<&str> = include_str!("../../resources/nynorsk/last_b.csv").split("\n").filter(|e| !!e.contains("name")).filter(|e| !e.contains("part")).collect();

    return combine_vectors(part1, part2);
}

// #[test]
// fn test_verify_get_all_female_names_is_valid(){
//     let names = get_all_female_names();

//     assert!(!names.is_empty());
//     assert!(names.len() >= 132);
// }


#[test]
fn test_generate_female_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_female();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);
}


#[test]
fn test_generate_male_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_male();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);
}

#[test]
fn test_generate_last_name_is_not_empty(){
    let name = RandomNynorskGenerator::generate_last_name();

    println!("Name: {}", name);

    assert!(name.is_empty() == false);
}