use names::{animal::AnimalName, animal::RandomAnimalGenerator, common::{RandomNameGenerator, Person, RandomGenderedNameGenerator, Gender}, nynorsk::RandomNynorskGenerator, norse::RandomNorseGenerator};


pub mod names;

//TODO: Expand with type of names to create
// Animal
// Norse
// Norwegian bokmål
// Norwegian Nynorsk


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
}