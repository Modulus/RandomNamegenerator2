use crate::RandomNameGenerator;
use crate::names::common;

use common::Person;

pub struct RandomNynorskGenerator {

}

impl RandomNameGenerator<Person> for RandomNynorskGenerator {

    fn create_rand_name() -> Person {

        let first = RandomNynorskGenerator::generate_female();
        let last = RandomNynorskGenerator::generate_last_name();

        return Person::new(&first, &last)
    }
}

impl RandomNynorskGenerator {

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