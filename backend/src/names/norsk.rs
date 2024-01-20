use crate::names::common;
use crate::names::common::RandomGenderedNameGenerator;
use crate::names::common::RandomNameGenerator;
use crate::names::common::Gender;
use common::Person;

pub struct RandomNorskGenerator{

}

impl RandomGenderedNameGenerator<Person> for RandomNorskGenerator {
    fn generate(gender: Gender) -> Person {
        return Person::new("", "");
    }

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