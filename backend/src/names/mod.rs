use std::fs;
use rand::Rng;
pub fn create_random(min: usize, max: usize) -> usize {
    return rand::thread_rng().gen_range(min..max);

}


fn return_random_element(words: Vec<&str>) -> Option<String> {
    if words.len() <= 0 {
        return None;
    }


    let filtered_names : Vec<String>= words.into_iter().filter(|e| !e.contains("name" )).filter(|e| !e.contains("adjective")) .map(String::from).collect();

    if filtered_names.len() <= 0 {
        return None;
    }

    match filtered_names.get(create_random(0, filtered_names.len())){
        Some(text) => Some(text.clone()),
        None => None
    }
}

pub trait RandomNameGenerator<T> {
    fn create_rand_name(&self) -> T;

}

# [derive(Debug)]
pub struct AnimalName {
    pub animal: String,
    pub adjective: String
}

pub struct RandonAnimalGenerator{

}

impl RandomNameGenerator<AnimalName> for RandonAnimalGenerator {
    fn create_rand_name(&self) -> AnimalName {
        let name = self.create_rand_animal_name().unwrap();
        let adjective = self.create_rand_adjective().unwrap();
        
        return AnimalName{animal: name, adjective}
        // if let (Some(name), Some(adjective)) = (n, a) {
        //     return AnimalName{animal: n, adjective: a}
        // }
        // return AnimalName { animal: String::from(""), adjective: String::from("") } 
    }
}

impl RandonAnimalGenerator {
    fn create_rand_animal_name(&self) -> Option<String> {
        let animal_names : Vec<&str> = include_str!("../../resources/animals.csv").split("\n").collect();

        return_random_element(animal_names)

    }

    fn create_rand_adjective(&self) -> Option<String> {
        let words : Vec<&str> = include_str!("../../resources/adjectives.csv").split("\n").collect();

        return_random_element(words)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_create_rand_is_in_range(){
        let num = create_random(0, 100);

        assert!(num <= 100);
    }

    #[test]
    fn test_vec_containing_excluded_words_should_return_empty_string(){
        let words = vec!["name", "adjective"];

        let element = return_random_element(words);

        assert!(element.is_none());

    }

        // Returns a random element from a non-empty vector of strings
    #[test]
    fn test_returns_random_element_from_non_empty_vector() {
        let words = vec!["apple", "banana", "orange"];
        let result = return_random_element(words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given an empty vector of strings
    #[test]
    fn test_returns_empty_string_with_empty_vector() {
        let words: Vec<&str> = vec![];
        let result = return_random_element(words);
        assert!(result.is_none());
    }
    
        // Filters out elements containing "name" or "adjective" before returning a random element
    #[test]
    fn test_filters_out_elements_containing_name_or_adjective() {
        let words = vec!["apple", "banana", "orange", "name", "adjective"];
        let result = return_random_element(words).unwrap();
        assert!(result == "apple" || result == "banana" || result == "orange");
    }
    
        // Returns an empty string when given a vector with only elements containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_only_name_or_adjective() {
        let words = vec!["name", "adjective", "name", "adjective"];
        let result = return_random_element(words);
        assert!(result.is_none());
    }
    
        // Returns an empty string when given a vector with only one element containing "name" or "adjective"
    #[test]
    fn test_returns_empty_string_with_vector_containing_one_name_or_adjective() {
        let words = vec!["name"];
        let result = return_random_element(words);
        assert!(result.is_none());
    }
    
        // Returns a string when given a vector with only one element not containing "name" or "adjective"
    #[test]
    fn test_returns_string_with_vector_containing_one_non_name_or_adjective() {
        let words = vec!["apple"];
        let result = return_random_element(words).unwrap();
        assert_eq!(result, "apple");
    }
}
