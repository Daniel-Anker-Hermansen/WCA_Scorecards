use wca_scorecards_lib::*;
#[allow(unused)]
use std::env::args;

fn main() {
    let mut args = args();
    let command = args.nth(1);
    match command {
        Some(v) if &v == "--r1" => {
            match (args.next(), args.next(), args.next()) {
                (Some(group_csv), Some(limit_csv), Some(competition)) => print_round_1_english(&group_csv, &limit_csv, &competition),
                _ => panic!("Malformatted r1 arguments")
            }
            unimplemented!()
        }
        Some(v) if &v == "--subseq" => {
            match args.next() {
                Some(id) => print_subsequent_rounds(id),
                None => panic!("No competition id given for subsequent rounds")
            }
        }
        Some(_) => panic!("Invalid command given"),
        None => panic!("No command given")
    }
}