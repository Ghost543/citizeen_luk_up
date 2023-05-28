use std::convert::From;
use std::error::Error;

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

fn main() {
    let cli = Cli::parse();
    let nin = cli.nin;
    println!("n_i_n: {:?}", nin);
    println!("n_i_n: {:?}", cli.surname.as_deref());

    let res = reading_file("src/data/LAKE  VICTORIA SCHOOL.xlsx").unwrap();
    if let Some(n_i_n) = nin {
        let dd = res.rows().find(|r| r[0].get_string().unwrap() == n_i_n);
        println!("{:?}", dd.unwrap());
        println!(
            "{:?}",
            res.rows().collect::<Vec<_>>()[1][0].get_string().unwrap()
        );
    }
}
