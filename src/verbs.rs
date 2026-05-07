use std::fmt::Display;
use crate::common::Numero;

#[derive(Clone, Copy)]
pub enum Modo {
    Indicativo,
    Congiuntivo,
    Imperativo,
    Infinito,

    #[allow(nonstandard_style)]
    __Modo_count
}


#[derive(Clone, Copy,PartialEq)]
pub enum Tempo{
    Presente,
    Imperfetto,
    Perfetto,
    Piucheperfetto,
    Futuro,
    FuturoAnteriore,

    __Count
}

#[derive(Clone, Copy)]
pub enum Coniugazione{
    I,
    II,
    III,
    IV,

    __Count
}

#[derive(Clone, Copy)]
pub enum Persona{
    Prima,
    Seconda,
    Terza,

    __Count
}

#[allow(dead_code)]
pub enum VerbsError{
    ConiugazioneNotFound,
    TempoNotFound,
    ImpossibleRequest
}


#[allow(dead_code)]
#[derive(Default)]
pub struct Paradigma<'a>{
    tempi: [&'a str; 5],
}

#[allow(dead_code)]
impl<'a> Paradigma<'a>
{
    pub fn new(tempi: &'a [String; 5]) -> Self{
        Self { tempi:[
            &tempi[0],
            &tempi[1],
            &tempi[2],
            &tempi[3],
            &tempi[4],
        ]}
    }
    fn get_coniugazione(&self) -> Result<usize, VerbsError>
    {
        let indic_presente = self.tempi[0];
        let i_len = indic_presente.len();
        let mut indic_presente = indic_presente.chars();

        let coniugazioni = &INDICATIVO.presente.coniugazioni;

        for (idx,c) in coniugazioni.iter().enumerate(){
            let suffix_prima_persona_singolare = c[0];
            let mut valid = true;

            for (idx, c_suffix) in suffix_prima_persona_singolare.chars().enumerate()
            {
                match indic_presente.nth(i_len - 1 - idx){
                    Some(c) => {
                        if c != c_suffix {
                            valid = false;
                            break;
                        }
                    },
                    None => {
                        valid = false;
                        break;
                    },
                }
            }

            if valid{
                return Ok(idx)
            }
        }

        Err(VerbsError::ConiugazioneNotFound)
    }



    pub fn coniuga_verbo(&self, modo: Modo, tempo: Tempo, persona: Persona, numero: Numero)
        -> Result<String, VerbsError>
    {
        let mut res = String::new();

        //TODO: add check irregular verbs

        let coniugazione = self.get_coniugazione()?;
        let forma = FORME_VERBALI[usize::from(modo)];
        let suffix = forma.get_suffix_verb(coniugazione, tempo, persona, numero)?;

        res.push_str(self.tempi[0]);
        res.pop(); //INFO: remove the Indicativo presente suffix (one letter)
        res.push_str(suffix);

        Ok(res)
    }
}

type Congiugazione<'a, const N: usize> = [&'a str; N];

struct FormaVerbale<'a, const N: usize>{
    coniugazioni: [Congiugazione<'a, N>; Coniugazione::__Count as usize],
}

trait InterfacciaVerbale {
    fn get_suffix_verb<'a> (&self, coniugazione: usize,  tempo: Tempo, persona: Persona,
        numero: Numero) -> Result<&'a str, VerbsError>;
}


type FormaVerbaleAttiva<'a> = FormaVerbale<'a, 6>;

struct Indicativo<'a>{
    presente: FormaVerbaleAttiva<'a>,
    imperfetto: FormaVerbaleAttiva<'a>,
    futuro: FormaVerbaleAttiva<'a>,

    perfetto: FormaVerbaleAttiva<'a>,
    piucheperfetto: FormaVerbaleAttiva<'a>,
    futuro_anteriore: FormaVerbaleAttiva<'a>,
}

#[allow(dead_code)]
struct Congiuntivo<'a>{
    presente: FormaVerbaleAttiva<'a>,
    imperfetto: FormaVerbaleAttiva<'a>,

    perfetto: FormaVerbaleAttiva<'a>,
    piucheperfetto: FormaVerbaleAttiva<'a>,
}

struct Imperativo<'a>{
    presente: FormaVerbale<'a, 2>,
    futuro: FormaVerbale<'a, 4>,
}

#[allow(dead_code)]
struct Infinito<'a>{
    presente: FormaVerbale<'a, 1>,
    perfetto: FormaVerbale<'a, 1>,
    // futuro: FormaVerbale<'a, 1>, INFO: future is weird: not adding at the moment
}

struct InvalidForma;

const INDICATIVO : Indicativo = Indicativo{
    presente: FormaVerbaleAttiva {
        coniugazioni : [
            ["o", "as", "at", "amus", "atis", "ant"],
            ["eo", "es", "et", "emus", "etis", "ent"],
            ["o", "is", "it", "imus", "itis", "unt"],
            ["io", "is", "it", "imus", "itis", "iunt"]
        ],
    },
    imperfetto: FormaVerbaleAttiva {
        coniugazioni: [
            ["abam", "abas", "abat", "abamus", "abatis", "abant"],
            ["ebam", "ebas", "ebat", "ebamus", "ebatis", "ebant"],
            ["ebam", "ebas", "ebat", "ebamus", "ebatis", "ebant"],
            ["iebam", "iebas", "iebat", "iebamus", "iebatis", "iebant"],
        ],
    },
    futuro: FormaVerbaleAttiva {
        coniugazioni:[
            ["abo", "abis", "abit", "abimus", "abitis", "abint"],
            ["ebo", "ebis", "ebit", "ebimus", "ebitis", "ebunt"],
            ["am", "es", "et", "emus", "etis", "ent"],
            ["iam", "ies", "iet", "iemus", "ietis", "ient"],
        ],
    },
    perfetto: FormaVerbaleAttiva {
        coniugazioni:[
            ["i", "isti", "it", "imus", "istis", "erunt"],
            ["i", "isti", "it", "imus", "istis", "erunt"],
            ["i", "isti", "it", "imus", "istis", "erunt"],
            ["i", "isti", "it", "imus", "istis", "erunt"],
        ],
    },
    piucheperfetto: FormaVerbaleAttiva {
        coniugazioni:[
            ["eram", "eras", "erat", "eramus", "eratis", "erant"],
            ["eram", "eras", "erat", "eramus", "eratis", "erant"],
            ["eram", "eras", "erat", "eramus", "eratis", "erant"],
            ["eram", "eras", "erat", "eramus", "eratis", "erant"],
        ],
    },
    futuro_anteriore: FormaVerbaleAttiva {
        coniugazioni:[
            ["ero", "eris", "erit", "erimus", "eritis", "erint"],
            ["ero", "eris", "erit", "erimus", "eritis", "erint"],
            ["ero", "eris", "erit", "erimus", "eritis", "erint"],
            ["ero", "eris", "erit", "erimus", "eritis", "erint"],
        ],
    },
};

const IMPERATIVO : Imperativo = Imperativo{
    presente: FormaVerbale {
        coniugazioni:[
            ["a", "ate"],
            ["e", "ete"],
            ["e", "ite"],
            ["i", "ite"]
        ],
    },
    futuro: FormaVerbale {
        coniugazioni:[
            ["ato", "ato", "atote","anto"],
            ["eto", "eto", "etote", "ento"],
            ["ito","ito", "itate", "unto"],
            ["ito","ito", "itote", "iunto"]
        ],
    },
};

const NOT_IMPLEMENTED : InvalidForma = InvalidForma;

#[allow(dead_code)]
const INFINITO: Infinito= Infinito{
    presente: FormaVerbale { coniugazioni: [["are"], ["ere"], ["ere"], ["ire"]] },
    perfetto: FormaVerbale { coniugazioni: [["isse"], ["isse"], ["isso"], ["isse"]]},
};

const FORME_VERBALI : [&dyn InterfacciaVerbale; Modo::__Modo_count as usize] = [
    &INDICATIVO,
    &NOT_IMPLEMENTED,
    &NOT_IMPLEMENTED,
    &IMPERATIVO,
];

impl From<Tempo> for i32{
    fn from(value: Tempo) -> Self {
        value as i32 
    }
}

impl From<Persona> for usize{
    fn from(value: Persona) -> Self {
        value as usize
    }
}

impl From<Modo> for usize{
    fn from(value: Modo) -> Self {
        value as usize
    }
}

impl From<Tempo> for usize{
    fn from(value: Tempo) -> Self {
        value as usize
    }
}

impl From<usize> for Modo{
    fn from(value: usize) -> Self {
        match value{
            0 => Self::Indicativo,
            1 => Self::Congiuntivo,
            2 => Self::Imperativo,
            3 => Self::Infinito,
            _ => unreachable!()
        }
    }
}

impl From<usize> for Tempo{
    fn from(value: usize) -> Self {
        match value{
            0 => Tempo::Presente,
            1 => Tempo::Imperfetto,
            2 => Tempo::Perfetto,
            3 => Tempo::Piucheperfetto,
            4 => Tempo::Futuro,
            5 => Tempo::FuturoAnteriore,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Persona{
    fn from(value: usize) -> Self {
        match value{
            0 => Persona::Prima,
            1 => Persona::Seconda,
            2 => Persona::Terza,
            3 => Persona::__Count,
            _ => unreachable!(),
        }
    }
    // add code here
}

impl Display for Modo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Modo::Indicativo => "Indicativo",
            Modo::Congiuntivo => "Congiuntivo",
            Modo::Imperativo => "Imperativo",
            Modo::Infinito => "Infinito",
            Modo::__Modo_count => unreachable!(),
        })
    }
    // add code here
}

impl Display for Tempo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tempo::Presente => "Presente",
            Tempo::Imperfetto => "Imperfetto",
            Tempo::Perfetto => "Perfetto",
            Tempo::Piucheperfetto => "Piucheperfetto",
            Tempo::Futuro => "Futuro",
            Tempo::FuturoAnteriore => "FuturoAnteriore",
            Tempo::__Count => unreachable!(),
        })
    }
    // add code here
}

impl Display for Persona{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Persona::Prima => "Prima",
            Persona::Seconda => "Seconda",
            Persona::Terza => "Terza",
            Persona::__Count => unreachable!(),
        })
    }
    // add code here
}

impl Display for VerbsError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            VerbsError::ConiugazioneNotFound => "ConiugazioneNotFound",
            VerbsError::TempoNotFound => "TempoNotFound",
            VerbsError::ImpossibleRequest => "ImpossibleRequest",
        })
    }
    // add code here
}

fn check_coniugazione<'b, const N: usize>(
    tempo_ref: &FormaVerbale<'b, N>,
    coniugazione: usize,
    persona: Persona,
    numero: Numero)
-> Result<&'b str, VerbsError>
{
    let idx = usize::from(persona) * 3 + usize::from(numero);
    if coniugazione >= tempo_ref.coniugazioni.len() {
        return Err(VerbsError::ConiugazioneNotFound)
    }
    let coniugazione = &tempo_ref.coniugazioni[coniugazione];

    if idx >= coniugazione.len()
    {
        return Err(VerbsError::ConiugazioneNotFound)
    }
    Ok(coniugazione[idx])
}

impl InterfacciaVerbale for Indicativo<'_>{
    fn get_suffix_verb<'a> (&self, coniugazione: usize, tempo: Tempo, persona: Persona,
        numero: Numero) -> Result<&'a str, VerbsError>
    {
        let tempo_ref = match tempo {
            Tempo::Presente => &INDICATIVO.presente,
            Tempo::Imperfetto => &INDICATIVO.imperfetto,
            Tempo::Perfetto => &INDICATIVO.perfetto,
            Tempo::Piucheperfetto => &INDICATIVO.piucheperfetto,
            Tempo::Futuro => &INDICATIVO.futuro,
            Tempo::FuturoAnteriore => &INDICATIVO.futuro_anteriore,
            Tempo::__Count => unreachable!(),
        };

        check_coniugazione(tempo_ref, coniugazione, persona, numero)
    }
}

impl InterfacciaVerbale for Imperativo<'_>{
    fn get_suffix_verb<'a> (&self, coniugazione: usize, tempo: Tempo, persona: Persona,
        numero: Numero) -> Result<&'a str, VerbsError>
    {
        match tempo {
            Tempo::Presente =>check_coniugazione(&IMPERATIVO.presente, coniugazione, persona, numero),
            Tempo::Futuro =>check_coniugazione(&IMPERATIVO.futuro, coniugazione, persona, numero),
            _ => Err(VerbsError::TempoNotFound),
        }
    }
}

impl InterfacciaVerbale for Infinito<'_>{
    fn get_suffix_verb<'a> (&self, coniugazione: usize, tempo: Tempo, persona: Persona,
        numero: Numero) -> Result<&'a str, VerbsError>
    {
        match tempo {
            Tempo::Presente =>check_coniugazione(&INFINITO.presente, coniugazione, persona, numero),
            Tempo::Perfetto=>check_coniugazione(&INFINITO.perfetto, coniugazione, persona, numero),
            _ => Err(VerbsError::TempoNotFound),
        }
    }
}

impl InterfacciaVerbale for InvalidForma{
    fn get_suffix_verb<'a> (&self, _coniugazione: usize, _tempo: Tempo, _persona: Persona,
        _numero: Numero) -> Result<&'a str, VerbsError>
    {
        Err(VerbsError::ImpossibleRequest)
    }
}

impl Display for Paradigma<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{},{}",
            self.tempi[0],
            self.tempi[1],
            self.tempi[2],
            self.tempi[3],
            self.tempi[4])
    }
    // add code here
}

impl From<Coniugazione> for usize{
    fn from(value: Coniugazione) -> Self {
        value as Self
    }
}

impl From<usize> for Coniugazione{
    fn from(value: usize) -> Self {
        match value {
            1 => Coniugazione::I,
            2 => Coniugazione::II,
            3 => Coniugazione::III,
            4 => Coniugazione::IV,
            5 => Coniugazione::__Count,

            _ => unreachable!()
        }
    }
    // add code here
}
