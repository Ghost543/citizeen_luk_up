pub mod data;
pub mod fields;

use std::convert::From;

use crate::data::data_controller::{get_file, merging_content_in_files};
use crate::fields::name::{
    getting_results_given_name, getting_results_name, getting_results_surname,
};
use crate::fields::nin::{getting_person, nin_validator, Nin};
use clap::Parser;

#[derive(Parser)]
#[command(author="Peter", version, about="Well lookup anyone", long_about=None)]
struct Cli {
    #[arg(long, short='c', value_parser=nin_validator)]
    nin: Option<Nin>,
    #[arg(long, short = 'n')]
    name: Option<String>,
    #[arg(long, short = 's')]
    surname: Option<String>,
    #[arg(long, short = 'g')]
    given_name: Option<String>,
}

//using a nin

//TODO: reducer function
// fn reducer<T, U, R>(comm: Command, action: Argument<T, U>) -> R {
//     match comm {
//         Command::Nin => getting_person(action.search, action.data),
//         Command::Name => getting_results_name(action.search, action.data),
//         Command::Surname => getting_results_name(action.search, action.data),
//         Command::GivenName => getting_results_name(action.search, action.data),
//     }
// }

fn main() {
    let cli = Cli::parse();
    let nin = cli.nin;
    let name = cli.name;
    let surname = cli.surname;
    let given_name = cli.given_name;

    let data = merging_content_in_files(&get_file());

    if let Some(nin) = nin {
        let result = getting_person(&nin, data.clone());
        println!("{:?}", result);
    }
    if let Some(name) = name {
        let results = getting_results_name(name.as_str(), data.clone());
        println!("{:?}", results);
    }
    if let Some(surname) = surname {
        let results = getting_results_surname(&surname, data.clone());
        println!("{:?}", results);
    }
    if let Some(given_name) = given_name {
        let results = getting_results_given_name(&given_name, data);
        println!("{:?}", results);
    }
}
