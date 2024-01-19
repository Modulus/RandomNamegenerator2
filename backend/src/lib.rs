use names::{animal::AnimalName, animal::RandomAnimalGenerator, common::{RandomNameGenerator, Person, RandomGenderedNameGenerator, Gender}, nynorsk::RandomNynorskGenerator};


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
        return RandomAnimalGenerator::create_rand_name();
    } 

    pub fn generate_nynorsk(gender: Gender) -> Person {
        return RandomNynorskGenerator::create_rand_name(gender);
    }
}