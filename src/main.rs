use std::env::args;
use pdf::run;
use language::Language;

mod language;
mod pdf;

fn main() {
    run(&mut args(), Language::english());
}