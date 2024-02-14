
use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

pub struct RandomNorseGenerator {

}

impl RandomGenderedNameGenerator<Person> for RandomNorseGenerator {
    fn generate(gender: Gender) -> Option<Person> {
        match gender {
            Gender::MALE => {
                return generate_male();
           },
            Gender::FEMALE => {
                return generate_female();
            },
            Gender::RANDOM => {
                let gender = common::generate_random_gender();
                match gender {
                    Gender::MALE => {
                        return generate_male();
                   },
                    Gender::FEMALE => {
                        return generate_female();
                    },
                    _ => None,
                }
            },
        }

    }
}

fn generate_female() -> Option<Person> {

    let first = RandomNorseGenerator::generate_female();
    let last = RandomNorseGenerator::generate_female_last_name();

    Some(Person::new_gendered(&first?, &last?, Gender::FEMALE))
}

fn generate_male() -> Option<Person> {

    let first = RandomNorseGenerator::generate_male();
    let last = RandomNorseGenerator::generate_male_last_name();

    Some(Person::new_gendered(&first?, &last?, Gender::MALE))
}

impl RandomNorseGenerator {
    pub fn generate_female() -> Option<String>{
        let names : Vec<&str> = include_str!("../../resources/norse_female.csv").split("\n").collect();


        return common::get_random_element(&names);
    }

    pub fn generate_male() -> Option<String>{
        let names : Vec<&str> = include_str!("../../resources/norse_male.csv").split("\n").collect();

        return common::get_random_element(&names);
    }

    pub fn generate_female_last_name() -> Option<String> {
        let names : Vec<&str> = include_str!("../../resources/norse_female_last.csv").split("\n").collect();

        return common::get_random_element(&names);
    }

    pub fn generate_male_last_name() -> Option<String> {
        let names : Vec<&str> = include_str!("../../resources/norse_male_last.csv").split("\n").collect();

        return common::get_random_element(&names);
    }
}