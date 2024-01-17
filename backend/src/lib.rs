use names::{AnimalName, RandomNameGenerator};


pub mod names;



pub struct Generator{
    animal_generator: names::RandonAnimalGenerator

}

impl Generator {
    pub fn new() -> Self {
        Self {
            animal_generator : names::RandonAnimalGenerator{}
        }
    }
    pub fn generate(&self) -> AnimalName {
        return self.animal_generator.create_rand_name();
    } 
}