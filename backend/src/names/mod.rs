pub trait RandomSurname{
    fn create_rand_name() -> String;
}

pub trait RandomFirstname {
    fn create_rand_name() -> String;

}

pub struct Name {
    pub first: String,
    pub last: String
}

pub struct NameGenerator {
    // pub generate_random() -> Name
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
