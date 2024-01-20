use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::RandomNameGenerator;
use crate::names::common::Gender;
use common::Person;

pub struct RandomNorskGenerator{

}

impl RandomGenderedNameGenerator<Person> for RandomNorskGenerator {
    fn genrate(gender: Gender) -> Person {
        return Person::new("", "");
    }

}

impl RandomNorskGenerator {
    pub fn generate_female_name() -> String {
        let names = include_str!("../norwegian_girls.csv")..split("\n").map(String::from).collect();

        return common::get_random_element(names);
    }

    pub fn generate_male_name() -> String {
        let names = include_str!("../norwegian_male.csv")..split("\n").map(String::from).collect();

        return common::get_random_element(names);        
    }
    pub fn generate_last_name() -> String {
        let names = include_str!("../norwegian_last.csv")..split("\n").map(String::from).collect();

        return common::get_random_element(names);   
    }
}