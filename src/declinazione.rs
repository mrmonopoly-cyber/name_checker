use crate::common::{GeneralPradigma, Numero};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Casi {
    Nominativo,
    Genitivo,
    Dativo,
    Accusativo,
    Vocativo,
    Ablativo,

    __Count,
}

#[allow(dead_code)]
pub enum DeclinazioneError {
    Unknown,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Paradigma {
    nominativo: String,
    genitivo: String,
}

#[allow(dead_code)]
impl Paradigma {
    pub fn new(nominativo: String, genitivo: String) -> Self {
        Paradigma {
            nominativo,
            genitivo,
        }
    }

    pub fn nominativo(&self) -> &str {
        &self.nominativo
    }

    pub fn genitivo(&self) -> &str {
        &self.genitivo
    }

    pub fn get_declinazione<'a>(
        &self,
    ) -> Result<(usize, &'a BasicDeclinazione<'a>), DeclinazioneError> {
        for (num, dec) in DECLINAZIONI.iter().enumerate() {
            let nom_sing =
                dec.numero[usize::from(Numero::Singolare)][usize::from(Casi::Nominativo)];
            let mut valid = true;
            let s_len = nom_sing.len();
            let name_len = self.nominativo.len();
            let mut nom_sing = nom_sing.chars();
            let mut nams_nom = self.nominativo.chars();

            for idx in 0..s_len {
                if nom_sing.nth(s_len - 1 - idx) != nams_nom.nth(name_len - 1 - idx) {
                    valid = false;
                    break;
                }
            }

            if valid {
                return Ok((num, dec));
            }
        }

        Err(DeclinazioneError::Unknown)
    }

    pub fn declina(&self, caso: Casi, num: Numero) -> Result<String, DeclinazioneError> {
        let (_, dec) = self.get_declinazione()?;
        let mut res = match String::from_str(&self.nominativo) {
            Ok(s) => s,
            Err(_) => return Err(DeclinazioneError::Unknown),
        };
        res.pop();

        let suffix = dec.numero[usize::from(num)][usize::from(caso)];
        res.push_str(suffix);

        Ok(res)
    }
}

type Cases<'a> = [&'a str; Casi::__Count as usize];

pub struct BasicDeclinazione<'a> {
    numero: [Cases<'a>; 2],
}

const DECLINAZIONI: [BasicDeclinazione; 1] = [BasicDeclinazione {
    numero: [
        ["a", "ae", "ae", "am", "a", "a"],
        ["ae", "arum", "is", "as", "ae", "is"],
    ],
}];

impl From<Casi> for usize {
    fn from(value: Casi) -> Self {
        value as Self
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

impl Display for Casi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Casi::Nominativo => write!(f, "Nominativo"),
            Casi::Genitivo => write!(f, "Genitivo"),
            Casi::Dativo => write!(f, "Dativo"),
            Casi::Accusativo => write!(f, "Accusativo"),
            Casi::Vocativo => write!(f, "Vocativo"),
            Casi::Ablativo => write!(f, "Ablativo"),
            _ => unreachable!(),
        }
    }
}

impl Display for Paradigma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.nominativo, self.genitivo)
    }
    // add code here
}

impl GeneralPradigma for Paradigma {
    // add code here
}

impl Display for DeclinazioneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeclinazioneError::Unknown => write!(f, "Unknown"),
        }
    }
}
