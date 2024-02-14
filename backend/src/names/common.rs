use std::collections::HashSet;

use rand::{seq::SliceRandom, Rng};

#[derive(Debug, PartialEq, Eq )]
pub struct Person {
    pub first_name: String,
    pub sur_name: String,
    pub gender: Gender,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    MALE,
    FEMALE,
    RANDOM,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    ANIMAL,
    NYNORSK,
    NORSK,
    NORSE
}

impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self { first_name: String::from(first_name), sur_name: String::from(last_name), gender: Gender::RANDOM} 
    }

    pub fn new_gendered(first_name: &str, last_name: &str, gender: Gender) -> Self{
        Self { first_name: String::from(first_name), sur_name: String::from(last_name), gender: gender }
    }
}

pub trait RandomNameGenerator<T> {
    fn generate() -> T;
}

pub trait RandomGenderedNameGenerator<T> {
    fn generate(gender: Gender) ->  T;
}



pub fn get_random(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..=max)
}

pub fn get_random_element(words: &Vec<&str>) -> Option<String> {
    let excluded_words: HashSet<&str> = ["name", "adjective", "part"].iter().cloned().collect();

    let filtered_names: Vec<String> = words.into_iter()
        .filter(|&e| !excluded_words.contains(e))
        .map(|e| e.replace("\t", "").replace("\r", "").replace("\n", ""))
        .map(String::from)
        .collect();

    if filtered_names.is_empty() {
        return None;
    }

    filtered_names.choose(&mut rand::thread_rng()).cloned()
}

pub fn generate_random_gender() -> Gender {
    if rand::random() {
        Gender::MALE
    } else {
        Gender::FEMALE
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_create_rand_is_in_range(){
        let num = get_random(0, 100);

        assert!(num <= 100);
    }

    
    #[test]
    fn test_vec_containing_excluded_words_should_return_empty_string(){
        let words = vec!["name", "adjective", "part"];

        let element = get_random_element(&words);

        assert!(element.is_none());

    }

    #[test]
    fn test_vec_words_contains_invalid_characters_not_such_returned(){
        let words = vec!["first\t", "second\n", "third\r"];

        let element = get_random_element(&words);

        assert!(!element.is_none());

        let word = element.unwrap();
        assert!(!word.contains("\t"));
        assert!(!word.contains("\r"));
        assert!(!word.contains("\n"));

    }

        // Returns a random element from a non-empty vector of strings
    #[test]
    fn test_returns_random_element_from_non_empty_vector() {
        let words = vec!["apple", "banana", "orange"];
        let result = get_random_element(&words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given an empty vector of strings
    #[test]
    fn test_returns_empty_string_with_empty_vector() {
        let words: Vec<&str> = vec![];
        let result = get_random_element(&words);
        assert!(result.is_none());
    }
    
        // Filters out elements containing "name" or "adjective" before returning a random element
    #[test]
    fn test_filters_out_elements_containing_name_or_adjective() {
        let words = vec!["apple", "banana", "orange", "name", "adjective"];
        let result = get_random_element(&words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given a vector with only elements containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_only_name_or_adjective() {
        let words = vec!["name", "adjective", "name", "adjective"];
        let result = get_random_element(&words);
        assert!(result.is_none());
    }
    
        // Returns an empty string when given a vector with only one element containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_one_name_or_adjective() {
        let words = vec!["name"];
        let result = get_random_element(&words);
        assert!(result.is_none());
    }
    
        // Returns a string when given a vector with only one element not containing "name" or "adjective"
    #[test]
    fn test_returns_string_with_vector_containing_one_non_name_or_adjective() {
        let words = vec!["apple"];
        let result = get_random_element(&words).unwrap();
        assert_eq!(result, "apple");
    }

    #[test]
    fn test_get_random_zero_to_one_returns_zero_or_one(){
        let random = get_random(0, 1);

        assert!(random == 0 || random == 1);
    }

    #[test]
    fn test_generate_random_gender_returns_valid_values(){
        let gender = generate_random_gender();
        assert!(gender == Gender::MALE || gender == Gender::FEMALE);
    }

    
}
