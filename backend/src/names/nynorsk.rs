use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

use super::common::generate_random_gender;

pub struct RandomNynorskGenerator {

}

impl RandomGenderedNameGenerator<Person> for RandomNynorskGenerator {

    fn generate(gender: Gender) -> Option<Person> {

        match gender {
            Gender::MALE => {

                generate_male()   
            },
            Gender::FEMALE => {

                generate_female()   
            },
            Gender::RANDOM => {
                match generate_random_gender(){
                    Gender::MALE => generate_male(),
                    Gender::FEMALE => generate_female(),
                    Gender::RANDOM => None,
                }
            }
        }

    }
}

fn generate_female() -> Option<Person> {
    let first = RandomNynorskGenerator::generate_female();
    let last = RandomNynorskGenerator::generate_last_name();

    Some(Person::new_gendered(&first?, &last?, Gender::FEMALE))
        
}

fn generate_male() -> Option<Person> {
    let first = RandomNynorskGenerator::generate_male();
    let last = RandomNynorskGenerator::generate_last_name();

    Some(Person::new_gendered(&first?, &last?, Gender::MALE)) 

}

impl RandomNynorskGenerator {
    pub fn generate_male() -> Option<String> {
        let first_male_names_a : Vec<&str> = include_str!("../../resources/nynorsk/first_male_a.csv").split("\n").collect();
        let first_male_names_b : Vec<&str> = include_str!("../../resources/nynorsk/first_male_b.csv").split("\n").collect();

        let first = common::get_random_element(&first_male_names_a);
        let last = common::get_random_element(&first_male_names_b);

        Some(format!("{}{}", first?, last?))

    }

    pub fn generate_female() -> Option<String> {
        let first_female_names_a : Vec<&str> = include_str!("../../resources/nynorsk/first_female_a.csv").split("\n").collect();
        let first_female_names_b : Vec<&str> = include_str!("../../resources/nynorsk/first_female_b.csv").split("\n").collect();

        let first = common::get_random_element(&first_female_names_a);
        let last = common::get_random_element(&first_female_names_b);

        Some(format!("{}{}", &first?, &last?))

    }
    
    pub fn generate_last_name() -> Option<String> {
        let first_parts : Vec<&str> = include_str!("../../resources/nynorsk/last_a.csv").split("\n").collect();
        let last_parts : Vec<&str> = include_str!("../../resources/nynorsk/last_b.csv").split("\n").collect();

        let first = common::get_random_element(&first_parts);
        let last = common::get_random_element(&last_parts);
        Some(format!("{}{}", first?, last?))

    }

}