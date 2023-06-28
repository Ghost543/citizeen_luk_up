use calamine::DataType;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Nin {
    prefix: Prefix,
    stem: (u8, u8, u8, u8, u8),
    suffix: String,
}

#[derive(Debug, Clone)]
pub struct Prefix {
    gender: String,
    yob: (u8, u8),
}

impl Prefix {
    pub fn new(s: String) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let (gender, yob) = s.split_at(2);
        match gender.to_lowercase().contains('c')
            && (gender.to_lowercase().contains('f') || gender.to_lowercase().contains('m'))
        {
            true => match yob.trim().parse::<u8>() {
                Ok(_val) => {
                    let gg: Vec<u8> = yob
                        .trim()
                        .chars()
                        .map(|x| x.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>();
                    Ok(Self {
                        gender: gender.to_uppercase(),
                        yob: (gg[0], gg[1]),
                    })
                }
                Err(_r) => Err(From::from("Invalid nin")),
            },
            false => Err(From::from("Invalid nin")),
        }
    }
}

impl Nin {
    pub fn new(s: String) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        if s.trim().len() != 14 {
            return Err(From::from("Invalid nin ** Characters should be 14 **"));
        }
        let (prefix, rest) = s.trim().split_at(4);
        let (stem, suffix) = rest.split_at(5);
        match Prefix::new(prefix.to_string()) {
            Ok(val) => {
                let gg: Vec<u8> = stem
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();
                Ok(Self {
                    prefix: val,
                    stem: (gg[0], gg[1], gg[2], gg[3], gg[4]),
                    suffix: suffix.to_uppercase(),
                })
            }
            Err(err) => Err(err),
        }
    }
}

pub fn nin_validator(s: &str) -> Result<Nin, Box<dyn Error + Send + Sync + 'static>> {
    if s.len() != 14 {
        Err(From::from("A Nin has to be 14 characters long"))
    } else {
        let nin_result = Nin::new(s.to_string());
        match nin_result {
            Ok(nin) => Ok(nin),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.gender, self.yob.0, self.yob.1)
    }
}

impl fmt::Display for Nin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.prefix,
            self.stem.0,
            self.stem.1,
            self.stem.2,
            self.stem.3,
            self.stem.4,
            self.suffix
        )
    }
}

pub fn getting_person(search: &Nin, res: Vec<Vec<DataType>>) -> Option<Vec<DataType>> {
    res.into_iter()
        .find(|row| row[0].get_string() == Some(search.to_string().as_str()))
}
