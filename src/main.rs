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

fn merging_content_in_files(files: &Vec<PathBuf>) {
    let mut merged_rows = Vec::new();
    for file in files {
        // println!("\n ------------------------------------------------------------------------------------------------------------\n");
        // println!("{:?}", file);
        // println!("\n ------------------------------------------------------------------------------------------------------------\n");
        let range = reading_file(file.to_str().unwrap()).unwrap();
        // println!("{:?}", range.rows().collect::<Vec<_>>());
        // println!("\n ------------------------------------------------------------------------------------------------------------\n");
        for row in range.rows().collect::<Vec<_>>() {
            merged_rows.push(row);
        }
    }
    println!("{:?}", merged_rows);
}

fn getting_person(search: &Option<String>) {
    let res = reading_file("src/data/LAKE  VICTORIA SCHOOL.xlsx").unwrap();
    if let Some(n_i_n) = search {
        let dd = res.rows().find(|r| r[0].get_string().unwrap() == n_i_n);
        println!("{:?}", dd.unwrap());
        println!(
            "{:?}",
            res.rows().collect::<Vec<_>>()[1][0].get_string().unwrap()
        );
    }
}

fn main() {
    let cli = Cli::parse();
    let nin = cli.nin;
    println!("n_i_n: {:?}", nin);
    println!("n_i_n: {:?}", cli.surname.as_deref());

    getting_person(&nin);
}
