use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

use super::common::generate_random_gender;

pub struct RandomNorskGenerator{

}

impl RandomGenderedNameGenerator<Person> for RandomNorskGenerator {
    fn generate(gender: Gender) -> Person {
        match gender {
            Gender::MALE => {
                //TODO: FIX THIS
                return generate_male().unwrap();
            }
            Gender::FEMALE => {
                //TODO: FIX THIS
                return generate_female().unwrap();
            },
            Gender::RANDOM => {
                //TODO: FIX THIS
                match generate_random_gender() {
                    Gender::MALE => generate_male().unwrap(),
                    Gender::FEMALE => generate_female().unwrap(),
                    Gender::RANDOM => todo!(),
                    Gender::UNKNOWN => todo!(),
                }
            }
            Gender::UNKNOWN => todo!(),
        }

    }

}

fn generate_female() -> Option<Person> {
    let first = RandomNorskGenerator::generate_female_name();
    let second = RandomNorskGenerator::generate_last_name();
    return Some(Person::new_gendered(&first, &second, Gender::FEMALE));
}

fn generate_male() -> Option<Person> {
    let first = RandomNorskGenerator::generate_male_name();
    let second = RandomNorskGenerator::generate_last_name();
    return Some(Person::new_gendered(&first, &second, Gender::MALE));
}

//TODO: FIX
impl RandomNorskGenerator {
    pub fn generate_female_name() -> String {
        let names = include_str!("../../resources/norwegian_girls.csv").split("\n").collect();

        return common::get_random_element(names).unwrap();
    }

    pub fn generate_male_name() -> String {
        let names = include_str!("../../resources/norwegian_boys.csv").split("\n").collect();

        return common::get_random_element(names).unwrap();        
    }
    pub fn generate_last_name() -> String {
        let names = include_str!("../../resources/norwegian_last.csv").split("\n").collect();

        return common::get_random_element(names).unwrap();   
    }
}