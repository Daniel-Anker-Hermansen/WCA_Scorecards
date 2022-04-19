use pdf::run;
use language::Language;

pub mod language;
mod pdf;
pub mod wcif;

pub fn print_round_1<I>(args: &mut I) where I: Iterator<Item = String> {
    run(args, Language::english());
}

pub fn print_round_1_with_language<I>(args: &mut I, language: Language) where I: Iterator<Item = String> {
    run(args, language);
}