use std::fs;
use std::env;
use rand::Error;
use rand::Rng;
fn create_random(min: usize, max: usize) -> usize {
    return rand::thread_rng().gen_range(min..max);

}

fn read_random_line_in_file(file: &str) -> String {

        // TODO: Fix this ASAP ZULU
        let all : Vec<String> = fs::read_to_string(file).unwrap().lines().map(String::from).skip(1).collect();

        let num = create_random(0, all.len());

        println!("{:?}", all[num]);

        return all[num].clone()
}
pub trait RandomNameGenerator {
    fn create_rand_name(&self) -> String;

}

pub struct AnimalName {
    pub animal: String,
    pub adjective: String
}

pub struct RandonAnimalGenerator{

}

impl RandomNameGenerator for RandonAnimalGenerator {
    fn create_rand_name(&self) -> String {
        String::from("")
    }
}

impl RandonAnimalGenerator {
    fn create_rand_animal_name(&self) -> String {

        read_random_line_in_file("resources/animals.csv")
    }

    fn create_rand_adjective(&self) -> String {
        read_random_line_in_file("resources/adjectives.csv")
    }

    pub fn create_rand_animal(&self) -> AnimalName {
        let name = self.create_rand_animal_name();
        let adjective = self.create_rand_adjective();
        AnimalName{animal: name, adjective}
    }
}

pub struct NameGenerator {
    // pub generate_random() -> Name
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rand_name_name_gives_name() {
        let random_firstname_generator = RandonAnimalGenerator{};
        let name = random_firstname_generator.create_rand_animal_name();
        assert!(name.len() > 0);
        
    }

    #[test]
    fn test_create_random_adjective_gives_string(){
        let random_firstname_generator = RandonAnimalGenerator{};
        let adj = random_firstname_generator.create_rand_adjective();
        assert!(adj.len() > 0);
    }

    #[test]
    fn test_create_random_animal_gives_name(){
        let random_firstname_generator = RandonAnimalGenerator{};
        let compound = random_firstname_generator.create_rand_animal();

        assert!(compound.animal.len() > 1);
        assert!(compound.adjective.len() > 1);
    }

    #[test]
    fn test_create_rand_is_in_range(){
        let num = create_random(0, 100);

        assert!(num >= 0);
        assert!(num <= 100);
    }


}
