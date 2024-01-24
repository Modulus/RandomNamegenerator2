use crate::RandomNameGenerator;
use crate::names::common;


# [derive(Debug)]
pub struct AnimalName {
    pub animal: String,
    pub adjective: String
}

pub struct RandomAnimalGenerator{

}

impl RandomNameGenerator<AnimalName> for RandomAnimalGenerator {
    fn generate() -> AnimalName {
        let name = RandomAnimalGenerator::get_random_animal_name().unwrap();
        let adjective = RandomAnimalGenerator::get_random_adjective().unwrap();
        
        return AnimalName{animal: name, adjective}
    }
}

impl RandomAnimalGenerator {
    fn get_random_animal_name() -> Option<String> {
        let animal_names : Vec<&str> = include_str!("../../resources/animals.csv").split("\n").collect();
        
        

        common::get_random_element(animal_names)

    }

    fn get_random_adjective() -> Option<String> {
        let words : Vec<&str> = include_str!("../../resources/adjectives.csv").split("\n").collect();

        common::get_random_element(words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;


    #[test]
    fn test_os_(){
        println!("{}", env::consts::OS);
    }
}



