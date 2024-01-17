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
    fn create_rand_name() -> AnimalName {
        let name = RandomAnimalGenerator::create_rand_animal_name().unwrap();
        let adjective = RandomAnimalGenerator::create_rand_adjective().unwrap();
        
        return AnimalName{animal: name, adjective}
        // if let (Some(name), Some(adjective)) = (n, a) {
        //     return AnimalName{animal: n, adjective: a}
        // }
        // return AnimalName { animal: String::from(""), adjective: String::from("") } 
    }
}

impl RandomAnimalGenerator {
    fn create_rand_animal_name() -> Option<String> {
        let animal_names : Vec<&str> = include_str!("../../resources/animals.csv").split("\n").collect();

        common::return_random_element(animal_names)

    }

    fn create_rand_adjective() -> Option<String> {
        let words : Vec<&str> = include_str!("../../resources/adjectives.csv").split("\n").collect();

        common::return_random_element(words)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    


}