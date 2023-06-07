use std::convert::From;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use calamine::{open_workbook, DataType, Range, Reader, Xlsx, XlsxError};
use clap::Parser;

#[derive(Parser)]
#[command(author="Peter", version, about="Well lookup anyone", long_about=None)]
struct Cli {
    #[arg(long, short='n', value_parser=nin_validator)]
    nin: Option<String>,
    #[arg(long, short = 's')]
    surname: Option<String>,
}

fn nin_validator(s: &str) -> Result<String, Box<(dyn Error + Send + Sync + 'static)>> {
    if s.len() != 14 {
        Err(From::from("A NIN has to be 14 characters long"))
    } else {
        Ok(s.to_string())
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

fn getting_person(search: &Option<String>) -> Option<Vec<DataType>> {
    let res = merging_content_in_files(&get_file());
    if let Some(n_i_n) = search {
        res.into_iter()
            .find(|row| row[0].get_string() == Some(n_i_n.as_str()))
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();
    let nin = cli.nin;
    println!("n_i_n: {:?}", nin);
    println!("surname: {:?}", cli.surname.as_deref());

    println!(
        "{:?}",
        getting_person(&nin).unwrap()[1].get_string().unwrap()
    );
}
