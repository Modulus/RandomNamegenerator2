extern crate backend;

use backend::names::read_random_line_in_file;
use backend::names::RandonAnimalGenerator;
use backend::names::RandomNameGenerator;
use std::env;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
// use crate::names::read_random_line_in_file;
        // Returns a string from a random line in a file
    
    // Returns a string from a random line in a file

fn generate_full_file_name(file: &str) -> PathBuf {
    let cur_dir = env::current_dir().unwrap();
    println!("Current dir: {:?}", cur_dir);



    let full_file = Path::new(&cur_dir).join(&file);

    println!("Full file path is: {:?}", full_file.to_str());

    return full_file;
}

#[test]
fn test_returns_random_line() {
    let full_file = generate_full_file_name("file.txt");
    let file = full_file.to_str().unwrap();
    let content = "Line 1\nLine 2\nLine 3\n";
    fs::write(file, content).unwrap();


    let result = read_random_line_in_file(file);

    assert!(content.contains(&result));

    fs::remove_file(file).unwrap();
}

    // Handles files with multiple lines
#[test]
fn test_handles_multiple_lines() {
    let full_file = generate_full_file_name("file.text");
    let file = full_file.to_str().unwrap();
    let content = "Line 1\nLine 2\nLine 3\n";
    fs::write(file, content).unwrap();

    let result = read_random_line_in_file(file);

    assert!(content.contains(&result));

    fs::remove_file(file).unwrap();
}

    // Handles files with only one line
#[test]
fn test_handles_single_line() {
    let full_file = generate_full_file_name("file.text");
    let file = full_file.to_str().unwrap();
    let content = "Line 1\n";
    fs::write(file, content).unwrap();

    let result = read_random_line_in_file(file);

    assert_eq!(content.trim(), result);

    fs::remove_file(file).unwrap();
}

    // Handles non-existent files
#[test]
fn test_handles_nonexistent_file() {
    let full_file = generate_full_file_name("nonexistent.text");
    let file = full_file.to_str().unwrap();


    let result = read_random_line_in_file(file);

    assert_eq!("", result);
}

    // Handles files with only one line and no newline character
#[test]
fn test_handles_single_line_no_newline() {

    let full_file = generate_full_file_name("test.text");
    let file = full_file.to_str().unwrap();

    let content = "Line 1";
    fs::write(file, content).unwrap();

    let result = read_random_line_in_file(file);

    assert_eq!(content, result);

    fs::remove_file(file).unwrap();
}

    // Handles files with only empty lines
#[test]
fn test_handles_empty_lines() {
    let file = "test.txt";
    let content = "\n\n\n";
    fs::write(file, content).unwrap();

    let result = read_random_line_in_file(file);

    assert_eq!(content.trim(), result);

    fs::remove_file(file).unwrap();
}



#[test]
fn test_create_random_animal_gives_name(){
    let random_firstname_generator = RandonAnimalGenerator{};
    let compound = random_firstname_generator.create_rand_name();

    assert!(compound.animal.len() > 1);
    assert!(compound.adjective.len() > 1);
}




