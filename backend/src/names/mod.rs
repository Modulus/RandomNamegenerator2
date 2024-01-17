use std::fs;
use rand::Rng;
pub fn create_random(min: usize, max: usize) -> usize {
    return rand::thread_rng().gen_range(min..max);

}


fn return_random_element(words: Vec<&str>) -> String {
    if words.len() <= 0 {
        return String::from("");
    }

    let rand = create_random(0, words.len());

    let filtered_names : Vec<String>= words.into_iter().filter(|e| !e.contains("name" )).filter(|e| !e.contains("adjective")) .map(String::from).collect();

    match filtered_names.get(rand){
        Some(text) => text.clone(),
        None => String::from("")
    }
}
pub fn read_random_line_in_file(file: &str) -> String {

        match fs::read_to_string(file) {
            Ok(data) => {
                let all : Vec<String> = data.lines().map(String::from)
                    .filter(| e| !e.contains("name") || !e.contains("adjective")).collect();

                let num = create_random(0, all.len());

                println!("{:?}", all[num]);

                all[num].clone()
            },
            Err(error) => {
                println!("Failed becaus of error: {:?}", error);
                String::from("")    
            }
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
        let name = self.create_rand_animal_name();
        let adjective = self.create_rand_adjective();
        AnimalName{animal: name, adjective}
    }
}

impl RandonAnimalGenerator {
    fn create_rand_animal_name(&self) -> String {
        let animal_names : Vec<&str> = include_str!("../../resources/animals.csv").split("\n").collect();

        return_random_element(animal_names)

    }

    fn create_rand_adjective(&self) -> String {
        let words : Vec<&str> = include_str!("../../resources/adjectives.csv").split("\n").collect();
        println!("ADJ!!!!: {:?}", words);

        return_random_element(words)
    }
}


pub struct NameGenerator {
    // pub generate_random() -> Name
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

        assert_eq!(String::from(""), element);

    }
}
