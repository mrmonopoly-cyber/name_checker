use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Numero {
    Singolare,
    Plurale,

    __Num__Numero,
}

#[derive(Clone, Copy)]
pub enum DeclinazioneConiugazione {
    I,
    II,
    #[allow(clippy::upper_case_acronyms)]
    III,
    IV,

    __Count,
}

pub trait GeneralPradigma: Display {}

impl From<Numero> for usize {
    fn from(value: Numero) -> Self {
        value as usize
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

impl Display for Numero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numero::Singolare => write!(f, "Singolare"),
            Numero::Plurale => write!(f, "Plurale"),
            Numero::__Num__Numero => unreachable!(),
        }
    }
}

impl From<DeclinazioneConiugazione> for usize {
    fn from(value: DeclinazioneConiugazione) -> Self {
        value as Self
    }
}

impl From<usize> for DeclinazioneConiugazione {
    fn from(value: usize) -> Self {
        match value {
            1 => DeclinazioneConiugazione::I,
            2 => DeclinazioneConiugazione::II,
            3 => DeclinazioneConiugazione::III,
            4 => DeclinazioneConiugazione::IV,
            5 => DeclinazioneConiugazione::__Count,

            _ => unreachable!(),
        }
    }
    // add code here
}

impl Display for DeclinazioneConiugazione {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DeclinazioneConiugazione::I => "I",
                DeclinazioneConiugazione::II => "II",
                DeclinazioneConiugazione::III => "III",
                DeclinazioneConiugazione::IV => "IV",
                DeclinazioneConiugazione::__Count => unreachable!(),
            }
        )
    }
    // add code here
}
