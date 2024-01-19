use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::Gender;
use common::Person;

pub struct RandomNynorskGenerator {

}

impl RandomGenderedNameGenerator<Person> for RandomNynorskGenerator {

    fn create_rand_name(gender: Gender) -> Person {

        match gender {
            Gender::MALE => {

                let first = RandomNynorskGenerator::generate_male();
                let last = RandomNynorskGenerator::generate_last_name();
        
                return Person::newGendered(&first, &last, Gender::MALE)   
            },
            Gender::FEMALE => {

                let first = RandomNynorskGenerator::generate_female();
                let last = RandomNynorskGenerator::generate_last_name();
        
                return Person::newGendered(&first, &last, Gender::FEMALE)   
            },
            Gender::RANDOM => todo!(),
            Gender::UNKNOWN => todo!(),
        }

    }
}

impl RandomNynorskGenerator {

    pub fn generate_male() -> String {
        let first_male_names_a : Vec<&str> = include_str!("../../resources/nynorsk/first_male_a.csv").split("\n").collect();
        let first_male_names_b : Vec<&str> = include_str!("../../resources/nynorsk/first_male_b.csv").split("\n").collect();

        let first = common::return_random_element(first_male_names_a).unwrap();
        let last = common::return_random_element(first_male_names_b).unwrap();

        let compunded = format!("{}{}", first, last);

        return compunded;
    }

    pub fn generate_female() -> String {
        let first_female_names_a : Vec<&str> = include_str!("../../resources/nynorsk/first_female_a.csv").split("\n").collect();
        let first_female_names_b : Vec<&str> = include_str!("../../resources/nynorsk/first_female_b.csv").split("\n").collect();

        let first = common::return_random_element(first_female_names_a).unwrap();
        let last = common::return_random_element(first_female_names_b).unwrap();

        let compunded = format!("{}{}", first, last);

        return compunded;

    }
    
    pub fn generate_last_name() -> String {
        let first_parts : Vec<&str> = include_str!("../../resources/nynorsk/last_a.csv").split("\n").collect();
        let last_parts : Vec<&str> = include_str!("../../resources/nynorsk/last_b.csv").split("\n").collect();

        let first = common::return_random_element(first_parts).unwrap();
        let last = common::return_random_element(last_parts).unwrap();

        let compunded = format!("{}{}", first, last);

        return compunded;
    }

}