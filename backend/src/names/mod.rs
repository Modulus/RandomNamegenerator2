use std::fs;
use rand::Rng;
pub fn create_random(min: usize, max: usize) -> usize {
    return rand::thread_rng().gen_range(min..max);

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

        read_random_line_in_file("resources/animals.csv")
    }

    fn create_rand_adjective(&self) -> String {
        read_random_line_in_file("resources/adjectives.csv")
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
}
