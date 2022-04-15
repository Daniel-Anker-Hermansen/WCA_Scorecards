use std::env::args;
use pdf::run;
use language::Language;

mod language;
mod pdf;

fn main() {
    let mut args = args();
    let csv1 = args.nth(1).unwrap();
    let csv2 = args.next().unwrap();
    let comp = args.next().unwrap();
    run(&mut [csv1.as_str(), csv2.as_str(), comp.as_str()].iter(), Language::english());
}