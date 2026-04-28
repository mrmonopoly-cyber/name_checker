use std::fmt::Display;
use std::io::Write;

use rand::RngExt;

use crate::exercise::Exercise;

pub enum Declinazioni{
    Prima =0,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
enum Numero{
    Singolare,
    Plurale,

    __Num__Numero
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Casi{
    Nominativo,
    Genitivo,
    Dativo,
    Accusativo,
    Vocativo,
    Ablativo,


    __Num__Casi
}

#[allow(unused)]
pub struct Declinazione<'a>{
    singular_suffixes: [&'a str; Casi::__Num__Casi as usize],
    plural_suffixes: [&'a str; Casi::__Num__Casi as usize],
}

impl Exercise for Declinazione<'_>{
    fn run_exercise(&self) {
        let mut rng = rand::rng();

        let mut user_input = String::new();
        let mut case_to_check;
        loop{
            let case : Casi =rng.random_range(0..Casi::__Num__Casi.into()).into();
            let numero :Numero = rng.random_range(0..Numero::__Num__Numero.into()).into();
            match numero{
                Numero::Singolare => case_to_check = self.singular_suffixes[usize::from(case)],
                Numero::Plurale => case_to_check = self.plural_suffixes[usize::from(case)],
                _ => unreachable!(),
            };

            print!("tell the latin suffix for the I Declinazione case {}-{}: ", numero, case);
            std::io::stdout().flush().unwrap();

            user_input.clear();
            match ::std::io::stdin().read_line(&mut user_input){
                Err(e) => println!("error reading stdin: {e}"),
                Ok(_) => {
                    user_input.pop();
                    match user_input.as_str() == case_to_check{
                        true => println!("Good job"),
                        false => println!("incorrect {}: given {}, expected {}",
                            case, user_input, case_to_check),
                    }
                },
            }
        }
    }
    // add code here
}

impl From<usize> for Numero {
    fn from(value: usize) -> Self {
        match value {
            0 => Numero::Singolare,
            1 => Numero::Plurale,
            _ => unreachable!(),
            
        }
    }
}

impl From<Numero> for usize{
    fn from(value: Numero) -> Self {
        match value {
            Numero::__Num__Numero => unreachable!(),
            _ => value as usize,
        }
    }
}

impl Display for Numero{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numero::Singolare => write!(f, "Singolare"),
            Numero::Plurale => write!(f, "Plurale"),
            Numero::__Num__Numero => unreachable!(),
        }
    }
    // add code here
}

impl Display for Casi{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Casi::Nominativo => write!(f, "Nominativo"),
            Casi::Genitivo => write!(f, "Genitivo"),
            Casi::Dativo => write!(f, "Dativo"),
            Casi::Accusativo => write!(f, "Accusativo"),
            Casi::Vocativo => write!(f, "Vocativo"),
            Casi::Ablativo => write!(f, "Ablativo"),
            _ => panic!("unreachable code"),
        }
    }
}

impl From<usize> for Casi {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Nominativo,
            1 => Self::Genitivo,
            2 => Self::Dativo,
            3 => Self::Accusativo,
            4 => Self::Vocativo,
            5 => Self::Ablativo,
            _ => unreachable!(),
        }
    }
}

impl From<Declinazioni> for usize{
    fn from(val: Declinazioni) -> Self {
        val as usize
    }
}

impl From<Casi> for usize {
    fn from(value: Casi) -> Self {
        value as Self
    }
}

#[allow(unused)]
pub static PRIMA_DECLINAZIONE : Declinazione = Declinazione{
    singular_suffixes: 
        [
        "a",
        "ae",
        "ae",
        "am",
        "a",
        "a",
        ],
    plural_suffixes:
        [
        "ae",
        "arum",
        "is",
        "as",
        "ae",
        "is",
        ],
};
