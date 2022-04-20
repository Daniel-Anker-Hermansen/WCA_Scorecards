use scorecards::*;
use std::env::args;

fn main() {
    let json = std::fs::read_to_string("files/wcif.json").unwrap();

    print_event_round(json, "333", 2, 19);

    //print_round_1(&mut args().skip(1));
}