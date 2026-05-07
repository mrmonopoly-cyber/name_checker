use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Declinazioni{
    Prima,

    __Num__Declinazioni
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Numero{
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

#[allow(dead_code)]
pub enum DeclinazioneError {
    Unknown,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Paradigma<'a>{
    nominativo: &'a str,
    genitivo: &'a str,
}

#[allow(dead_code)]
impl<'b> Paradigma<'b>
{
    pub fn new(nominativo: &'b str, genitivo: &'b str) -> Self{
        Paradigma { nominativo, genitivo }
    }

    fn get_declinazione<'a>(&self) -> Result<&'a BasicDeclinazione<'a>, DeclinazioneError>{
        for dec in DECLINAZIONI.iter(){
            let nom_sing = dec.numero[usize::from(Numero::Singolare)][usize::from(Casi::Nominativo)];
            let mut valid = true;
            let s_len = nom_sing.len();
            let name_len = self.nominativo.len();
            let mut nom_sing = nom_sing.chars();
            let mut nams_nom = self.nominativo.chars();

            for idx in 0..s_len{
                if nom_sing.nth(s_len -1 - idx) != nams_nom.nth(name_len -1 -idx){
                    valid = false;
                    break;
                }
            }

            if valid{
                return Ok(dec);
            }
        }

        Err(DeclinazioneError::Unknown)
    }

    pub fn declina(&self, caso: Casi, num: Numero) -> Result<String, DeclinazioneError>{
        let dec = self.get_declinazione()?;
        let mut res = match String::from_str(self.nominativo){
            Ok(s) => s,
            Err(_) => return Err(DeclinazioneError::Unknown),
        };
        res.pop();

        let suffix = dec.numero[usize::from(num)][usize::from(caso)];
        res.push_str(suffix);

        Ok(res)
    }
}

type Cases<'a> = [&'a str; Casi::__Num__Casi as usize];

struct BasicDeclinazione<'a>{
     numero: [Cases<'a>;2],
}

const DECLINAZIONI : [BasicDeclinazione; 1] = 
[
    BasicDeclinazione{
        numero: [
            [
                "a",
                "ae",
                "ae",
                "am",
                "a",
                "a",
            ],
            [
                "ae",
                "arum",
                "is",
                "as",
                "ae",
                "is",
            ],
        ],
    },
];

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

impl From<Numero> for usize{
    fn from(value: Numero) -> Self {
        value as usize
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

impl From<usize> for Declinazioni{
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Prima,
            1 => Self::__Num__Declinazioni,
            _ => unreachable!(),
        }
    }
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

impl Display for Numero{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numero::Singolare => write!(f, "Singolare"),
            Numero::Plurale => write!(f, "Plurale"),
            Numero::__Num__Numero => unreachable!(),
        }
    }
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
            _ => unreachable!()
        }
    }
}

impl Display for Paradigma<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.nominativo, self.genitivo)
    }
    // add code here
}
