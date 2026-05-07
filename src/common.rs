use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, nonstandard_style, clippy::enum_variant_names)]
pub enum Numero{
    Singolare,
    Plurale,

    __Num__Numero
}

impl From<Numero> for usize{
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

impl Display for Numero{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Numero::Singolare => write!(f, "Singolare"),
            Numero::Plurale => write!(f, "Plurale"),
            Numero::__Num__Numero => unreachable!(),
        }
    }
}
