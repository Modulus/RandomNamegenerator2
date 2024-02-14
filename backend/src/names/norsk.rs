use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

use super::common::generate_random_gender;

pub struct RandomNorskGenerator{

}

impl RandomGenderedNameGenerator<Person> for RandomNorskGenerator {
    fn generate(gender: Gender) -> Option<Person> {
        match gender {
            Gender::MALE => {
                return generate_male();
            }
            Gender::FEMALE => {
                return generate_female();
            },
            Gender::RANDOM => {
                match generate_random_gender() {
                    Gender::MALE => generate_male(),
                    Gender::FEMALE => generate_female(),
                    Gender::RANDOM => None,
                }
            }
        }

    }

}

fn generate_female() -> Option<Person> {
    let first = RandomNorskGenerator::generate_female_name();
    let second = RandomNorskGenerator::generate_last_name();
    match (first, second) {
        (Some(first), Some(second)) => Some(Person::new_gendered(&first, &second, Gender::FEMALE)),
        (_,_) => None
    }
}

fn generate_male() -> Option<Person> {
    let first = RandomNorskGenerator::generate_male_name();
    let second = RandomNorskGenerator::generate_last_name();
    match (first, second) {
        (Some(first), Some(second)) => Some(Person::new_gendered(&first, &second, Gender::MALE)),
        (_,_) => None
    }
}

impl RandomNorskGenerator {
    pub fn generate_female_name() -> Option<String> {
        let names : Vec<&str> = include_str!("../../resources/norwegian_girls.csv").split("\n").collect();

        return common::get_random_element(&names);
    }

    pub fn generate_male_name() -> Option<String> {
        let names : Vec<&str> = include_str!("../../resources/norwegian_boys.csv").split("\n").collect();

        return common::get_random_element(&names);        
    }
    pub fn generate_last_name() -> Option<String> {
        let names : Vec<&str> = include_str!("../../resources/norwegian_last.csv").split("\n").collect();

        return common::get_random_element(&names);   
    }
}