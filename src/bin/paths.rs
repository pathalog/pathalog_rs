use lexpr::cons::ListIter;
use lexpr::parse;
use lexpr::Value;
use log::debug;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::io;

//A Path is defined by its own Label (String) and its contents (Vec<Path>)
#[derive(Debug, Clone)]
struct Path(String, Vec<Path>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (", self.0)?;
        match self.1.is_empty() {
            true => {
                write!(f, " \n")
            }
            false => {
                write!(f, "");
                for u in self.1.clone() {
                    &u.innerfmt(f, 1)?;
                    if u.len() > 0 {
                        //writeln!(f, "")?;
                    } else {
                        //writeln!(f, " => {}", 0)?;
                    }
                }
                Ok(())
            }
        }
    }
}


impl Iterator for Path {
    type Item = Path;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.pop()
    }
}

impl Path {
    fn len(&self) -> usize {
        self.1.len()
    }

    fn variants(&self) -> usize {
        let mut variants = 1;
        for item in self.1.clone() {
            variants = variants * item.len()+1
        }
        1
    }

    fn innerfmt(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, " {}", self.0)?;
        match self.1.is_empty() {
            true => {
                write!(f, " )");
                Ok(())
            },
            false => {
                write!(f, " (");
                for u in self.1.clone() {
                    &u.innerfmt(f, depth + 1)?;
                }
                match self.1.is_empty() {
                    true => write!(f, ""),
                    false => write!(f, "( "),
                }?;
                write!(f, "");
                // if depth < 2 {
                //     write!(f, " => {}", self.1.len())?;
                // }
                Ok(())
            }
        }
    }

    fn new(sexpr: Value) -> Result<Self, io::Error> {
        {
            Ok(match sexpr.clone() {
                Value::Nil | Value::Null => Path("".to_string(), vec![]),
                Value::Number(n) => Path(n.to_string(), vec![]),
                Value::Symbol(sym) => Path(sym.to_string(), vec![]),
                Value::Cons(item) => {
                    let contents: Vec<Path> = item
                        .cdr()
                        .list_iter()
                        .unwrap()
                        .map(|elem| Path::new(elem.clone()).unwrap())
                        .collect();
                    Path(item.car().to_string(), contents.into_iter().collect())
                }
                Value::Vector(_) => unimplemented!(),
                Value::Bool(b) => unimplemented!(),
                Value::Char(c) => unimplemented!(),
                Value::String(_) => unimplemented!(),
                Value::Keyword(_) => unimplemented!(),
                Value::Bytes(_) => unimplemented!(),
            })
        }
    }

    fn evaluate(&self, value: String) -> Result<(), parse::Error> {
        let value = lexpr::from_str(&value)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let u = Path::new(lexpr::from_str(&fs::read_to_string(
        "paths/Universe.paths",
    )?)?)
    .unwrap();

    // let u = Universe {
    //     label: "foou".to_string(),
    //     contents: vec![],
    // };
    //   u.print_tokens();
    //println!("{}", u);

    for path in u {
      println!("{}", path)
    }


    Ok(())

}
