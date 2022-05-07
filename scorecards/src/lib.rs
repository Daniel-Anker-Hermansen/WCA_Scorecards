use pdf::run;
use language::Language;

pub mod language;
mod pdf;
pub mod wcif;
mod localhost;

pub fn print_round_1<I>(args: &mut I) where I: Iterator<Item = String> {
    run(args, Language::english());
}

pub fn print_round_1_with_language<I>(args: &mut I, language: Language) where I: Iterator<Item = String> {
    run(args, language);
}

pub fn print_subsequent_rounds(competition_id: String) {
    localhost::init(competition_id);
}

#[allow(unused)]
#[deprecated]
pub fn print_event_round(id: &str, event: &str, round: usize, max_group_size: usize) {
    unimplemented!();
}