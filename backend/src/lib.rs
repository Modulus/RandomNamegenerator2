use names::{animal::AnimalName, animal::RandomAnimalGenerator, common::{RandomNameGenerator, Person, RandomGenderedNameGenerator, Gender}, nynorsk::RandomNynorskGenerator, norse::RandomNorseGenerator, norsk::RandomNorskGenerator};

pub mod names;

pub struct Generator{

}


impl Generator {

    pub fn generate() -> Option<AnimalName> {
        return RandomAnimalGenerator::generate();
    } 

    pub fn generate_nynorsk(gender: Gender) -> Option<Person> {
        return RandomNynorskGenerator::generate(gender);
    }

    pub fn generate_norse(gender: Gender) -> Option<Person> {
        return RandomNorseGenerator::generate(gender);
    }

    pub fn generate_norsk(gender: Gender) -> Option<Person> {
        return RandomNorskGenerator::generate(gender);
    }
}