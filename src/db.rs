use std::fmt::{Display};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum Casi{
    Nominativo = 0,
    Genitivo = 1,
}


#[derive(Debug, Clone)]
pub struct Name {
    pub italian: String,
    pub latin: [String;2],
}

pub struct DB{
    db: Vec::<Name>,
    it_i: usize,
}

impl DB{
    pub fn new(path : &std::path::Path) -> Result<Self, std::io::Error>
    {
        let mut res = Self{db: Vec::new(), it_i:0};
        let lines = match read_lines(path){
            Ok(l) => l,
            Err(e) => {
                return Err(e);
            },
        };

        for line in lines.map_while(Result::ok) {
            let words : Vec<&str> = line.split(':').collect();
            let italian = words[0].to_string();
            let latins = words[1].to_string();
            let mut split = latins.split(',');
            let ele = Name{
                italian: italian.to_string(),
                latin: [split.next().unwrap().to_string(), split.next().unwrap().to_string()],
            };

            res.db.push(ele);
        }
        Ok(res)
    }
    pub fn len(&self) -> usize{
        self.db.len()
    }

    pub fn at(&'_ self, i: usize) -> Option<&'_ Name>{
        self.db.get(i)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl Iterator for DB{
    type Item = Name;

    fn next(&mut self) -> Option<Self::Item> {
        if self.it_i < self.db.len(){
            let name = self.db[self.it_i].clone();
            self.it_i+=1;
            return Some(name);
        }
        None
    }
}

impl Display for Name{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:[{},{}]", self.italian, self.latin[0], self.latin[1])
    }
}

impl Display for Casi{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Casi::Nominativo => write!(f, "Nominativo"),
            Casi::Genitivo => write!(f, "Genitivo"),
        }
    }
}
