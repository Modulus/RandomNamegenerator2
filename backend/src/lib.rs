use names::{animal::AnimalName, animal::RandomAnimalGenerator, common::{RandomNameGenerator, Person, RandomGenderedNameGenerator, Gender}, nynorsk::RandomNynorskGenerator, norse::RandomNorseGenerator, norsk::RandomNorskGenerator};

pub mod names;

pub struct Generator{

}


impl Generator {

    pub fn generate() -> AnimalName {
        return RandomAnimalGenerator::generate();
    } 

    pub fn generate_nynorsk(gender: Gender) -> Person {
        return RandomNynorskGenerator::generate(gender);
    }

    pub fn generate_norse(gender: Gender) -> Person {
        return RandomNorseGenerator::generate(gender);
    }

    pub fn generate_norsk(gender: Gender) -> Person {
        return RandomNorskGenerator::generate(gender);
    }
}