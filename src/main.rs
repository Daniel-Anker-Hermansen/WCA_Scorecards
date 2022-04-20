use scorecards::*;
use std::env::args;

fn main() {
    let json = std::fs::read_to_string("files/wcif.json").unwrap();

    scorecards::wcif::get_advancement(json);

    //print_round_1(&mut args().skip(1));
}