use std::{path::PathBuf, str::FromStr};

use clap::{arg, value_parser, Command};

use crate::held_karp_iterativo::HeldKarp;
use crate::held_karp_recursivo::HeldKarpRecursivo;

mod grid;
mod held_karp_iterativo;
mod held_karp_recursivo;
mod tsp;

fn cli() -> Command {
    Command::new("tsp")
        .about("Tool for playing with the Traveling Salesman Problem")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
                .about("Create a new tsp map")
                .arg_required_else_help(true)
                .arg(arg!(<FILE> "Name of the file to be created"))
                .arg(
                    arg!(--size [SIZE] "Size of the grid")
                        .default_value("1000")
                        .value_parser(value_parser!(u32)),
                )
                .arg(
                    arg!(--cities [AMOUNT] "Amount of cities to be generated")
                        .default_value("20")
                        .value_parser(value_parser!(u32)),
                ),
        )
        .subcommand(
            Command::new("solve")
                .about("Solves a map")
                .arg_required_else_help(true)
                .arg(arg!(<FILE> "Name of the map file to solve"))
                .arg(arg!(--recursive "To use the recursive solution")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", submatches)) => {
            let file = submatches
                .get_one::<String>("FILE")
                .expect("Required argument");
            let size = submatches
                .get_one::<u32>("size")
                .expect("Argument has default value");
            let num_cities = submatches
                .get_one::<u32>("cities")
                .expect("Argument has default value");

            let filename = PathBuf::from_str(file).expect("Infallible");

            tsp::Tsp::random(*size as usize, *num_cities as usize)
                .save(&filename)
                .unwrap();
        }
        Some(("solve", submatches)) => {
            let file = submatches
                .get_one::<String>("FILE")
                .expect("Required argument");
            let recursive = submatches.get_flag("recursive");

            let filename = PathBuf::from_str(file).expect("Infallible");
            let tsp = tsp::Tsp::load(&filename).unwrap();
            let ans = if recursive {
                tsp.solve::<HeldKarpRecursivo>()
            } else {
                tsp.solve::<HeldKarp>()
            };
            println!("O menor caminho encontrado é {ans}");
        }
        _ => unreachable!(),
    }
}
