use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

pub struct RandomNorseGenerator {

}

impl RandomGenderedNameGenerator<Person> for RandomNorseGenerator {
    fn generate(gender: Gender) -> Person {
        return Person::new("first", "last"); 
    }
}