
use rand::Rng;

#[derive(Debug, PartialEq, Eq )]
pub struct Person {
    pub first_name: String,
    pub sur_name: String,
    pub gender: Gender,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Gender {
    MALE,
    FEMALE,
    RANDOM,
    UNKNOWN
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
        Self { first_name: String::from(first_name), sur_name: String::from(last_name), gender: Gender::UNKNOWN} 
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
    return rand::thread_rng().gen_range(min..max);

}


pub fn get_random_element(words: Vec<&str>) -> Option<String> {
    if words.is_empty() {
        return None;
    }


    let filtered_names : Vec<String> = words.into_iter()
        .filter(|e| !e.contains("name" ))
        .filter(|e| !e.contains("adjective"))
        .filter(|e| !e.contains("part")) 
        .map(String::from).collect();

    if filtered_names.is_empty() {
        return None;
    }

    match filtered_names.get(get_random(0, filtered_names.len())){
        Some(text) => Some(text.clone()),
        None => None
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

        let element = get_random_element(words);

        assert!(element.is_none());

    }

        // Returns a random element from a non-empty vector of strings
    #[test]
    fn test_returns_random_element_from_non_empty_vector() {
        let words = vec!["apple", "banana", "orange"];
        let result = get_random_element(words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given an empty vector of strings
    #[test]
    fn test_returns_empty_string_with_empty_vector() {
        let words: Vec<&str> = vec![];
        let result = get_random_element(words);
        assert!(result.is_none());
    }
    
        // Filters out elements containing "name" or "adjective" before returning a random element
    #[test]
    fn test_filters_out_elements_containing_name_or_adjective() {
        let words = vec!["apple", "banana", "orange", "name", "adjective"];
        let result = get_random_element(words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given a vector with only elements containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_only_name_or_adjective() {
        let words = vec!["name", "adjective", "name", "adjective"];
        let result = get_random_element(words);
        assert!(result.is_none());
    }
    
        // Returns an empty string when given a vector with only one element containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_one_name_or_adjective() {
        let words = vec!["name"];
        let result = get_random_element(words);
        assert!(result.is_none());
    }
    
        // Returns a string when given a vector with only one element not containing "name" or "adjective"
    #[test]
    fn test_returns_string_with_vector_containing_one_non_name_or_adjective() {
        let words = vec!["apple"];
        let result = get_random_element(words).unwrap();
        assert_eq!(result, "apple");
    }

    #[test]
    fn test_new_person_without_gendere_has_gender_unknown(){
        let person = Person::new("jadda", "jauda");

        assert_eq!(person.gender, Gender::UNKNOWN);
    }
}