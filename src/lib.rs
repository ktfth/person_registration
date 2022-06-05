use rand::{thread_rng, Rng};

pub trait Gen {
    fn generate() -> String;
}

pub struct Person;

impl Gen for Person {
    fn generate() -> String {
        let mut out = String::from("###.###.###-##");
        while out.find("#") != None {
            let mut rng = thread_rng();
            let num: u32 = rng.gen_range(0..10);
            out = out.replacen("#", &num.to_string(), 1);
        }
        out.to_string()
    }
}

pub struct Juridic;

impl Gen for Juridic {
    fn generate() -> String {
        let mut out = String::from("##.###.###/####-##");
        while out.find("#") != None {
            let mut rng = thread_rng();
            let num: u32 = rng.gen_range(0..10);
            out = out.replacen("#", &num.to_string(), 1);
        }
        out.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use regex::Regex;

    #[test]
    fn physical_person_registration() {
        let re = Regex::new(r"[0-9]{3}\.[0-9]{3}\.[0-9]{3}\-[0-9]{2}").unwrap();
        let mut expectations = vec![];
        for _i in 0..100 {
            let generated_value = Person::generate();
            if re.is_match(&generated_value) {
                expectations.push(generated_value);
            }
        }
        assert_eq!(expectations.len(), 100);
        let expect = expectations.binary_search(&Person::generate());
        assert!(match expect { Err(0..=100) => true, _ => false, });
    }

    #[test]
    fn juridic_person_registration() {
        let re = Regex::new(r"[0-9]{2}\.[0-9]{3}\.[0-9]{3}/[0-9]{4}\-[0-9]{2}").unwrap();
        let mut expectations = vec![];
        for _i in 0..100 {
            let generated_value = Juridic::generate();
            if re.is_match(&generated_value) {
                expectations.push(generated_value);
            }
        }
        assert_eq!(expectations.len(), 100);
        let expect = expectations.binary_search(&Juridic::generate());
        assert!(match expect { Err(0..=100) => true, _ => false, });
    }
}