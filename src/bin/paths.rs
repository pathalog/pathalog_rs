use lexpr::Value;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write};
use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct Path(String, Vec<Path>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.0)?;
        match self.1.is_empty() {
            true => {
                write!(f, "\n")
            }
            false => {
                write!(f, "=> ")?;
                let mut output = String::new();
                for u in self.1.clone() {
                    &mut output.push_str(&u.innerfmt(1)?);
                }
                write!(f, "( {})", output)?;
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
    fn innerfmt(&self, _depth: usize) -> Result<String, fmt::Error> {
        let mut output = String::new();
        match self.1.is_empty() {
            true => {
                write!(&mut output, "{} ", self.0)?;
                Ok(output)
            }
            false => {
                write!(output, "( {} )", self.0)?;
                for path in &self.1 {
                    &mut output.push_str(&path.innerfmt(1)?);
                }
                Ok(output)
            }
        }
    }

    fn new(sexpr: Value) -> Result<Self, io::Error> {
        {
            Ok(match sexpr.clone() {
                Value::Nil | Value::Null => Path("()".to_string(), vec![]),
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
                Value::Bool(_) => unimplemented!(),
                Value::Char(_) => unimplemented!(),
                Value::String(string) => Path(string.to_string(), vec![]),
                Value::Keyword(_) => unimplemented!(),
                Value::Bytes(_) => unimplemented!(),
            })
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut u: Vec<Path> = Path::new(lexpr::from_str(&fs::read_to_string(
        "paths/Universe.paths",
    )?)?)
    .unwrap()
    .collect();
    u.reverse();

    for path in u {
        println!("{}", path)
    }
    Ok(())
}
