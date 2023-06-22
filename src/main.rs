use std::convert::From;
use std::error::Error;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use std::{env, fmt};

use calamine::{open_workbook, DataType, Range, Reader, Xlsx, XlsxError};
use clap::Parser;

#[derive(Debug, Clone)]
struct Nin {
    prefix: Prefix,
    stem: (u8, u8, u8, u8, u8),
    suffix: String,
}

#[derive(Debug, Clone)]
struct Prefix {
    gender: String,
    yob: (u8, u8),
}

impl Prefix {
    fn new(s: String) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
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

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.gender, self.yob.0, self.yob.1)
    }
}

impl Nin {
    fn new(s: String) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
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

#[derive(Parser)]
#[command(author="Peter", version, about="Well lookup anyone", long_about=None)]
struct Cli {
    #[arg(long, short='n', value_parser=nin_validator)]
    nin: Option<Nin>,
    #[arg(long, short = 's')]
    surname: Option<String>,
    #[arg(long, short = 'g')]
    given_name: Option<String>,
}

fn nin_validator(s: &str) -> Result<Nin, Box<dyn Error + Send + Sync + 'static>> {
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

fn reading_file(s: &str) -> Result<Range<DataType>, XlsxError> {
    let mut execel: Xlsx<_> = open_workbook(s).unwrap();
    match execel.worksheet_range("Sheet1") {
        Some(val) => val,
        None => panic!("not an xlxs file"),
    }
}

fn get_file() -> Vec<PathBuf> {
    let dir = env::current_dir()
        .unwrap()
        .join(Path::new("src"))
        .as_path()
        .join(Path::new("data"));

    let mut files = Vec::new();

    for i in dir.read_dir().unwrap() {
        let file = i.unwrap();
        files.push(file.path());
    }
    files.sort();
    files
}

fn merging_content_in_files(files: &Vec<PathBuf>) -> Vec<Vec<DataType>> {
    let mut merged_rows = Vec::new();
    for file in files {
        let range = reading_file(file.to_str().unwrap()).unwrap();
        let rows: Vec<Vec<DataType>> = range.rows().map(|row| row.to_vec()).collect();
        merged_rows.extend(rows);
    }
    // println!("{:?}", merged_rows);
    merged_rows
}

fn getting_person(search: &Option<Nin>) -> Option<Vec<DataType>> {
    let res = merging_content_in_files(&get_file());
    if let Some(n_i_n) = search {
        res.into_iter()
            .find(|row| row[0].get_string() == Some(n_i_n.to_string().as_str()))
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();
    let nin = cli.nin;
    println!("n_i_n: {:?}", nin);
    println!("surname: {:?}", cli.surname.as_deref());

    // getting_person(&nin);
    println!(
        "{:?}",
        getting_person(&nin).unwrap()[1].get_string().unwrap()
    );
}
