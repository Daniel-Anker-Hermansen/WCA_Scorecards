use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use language::Language;
use pdf::run;

mod language;
mod pdf;

#[pymodule]
fn wca_scorecards(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(print_pdf_with_language, m)?)?;
    Ok(())
}

#[pyfunction]
fn print_pdf(_py: Python, csv1: &str, csv2: &str, comp: &str) -> PyResult<()> {
    run(&mut [csv1, csv2, comp].iter().map(|x|x.to_string()), Language::english());
    Ok(())
}

#[pyfunction]
fn print_pdf_with_language(_py: Python, csv1: &str, csv2: &str, comp: &str, language: &str) -> PyResult<()> {
    let mut lang = Language::english();
    let string = std::fs::read_to_string(language).unwrap();
    string.lines()
        .for_each(|x|{
            let mut iter = x.split(":");
            let first = iter.next();
            let second = iter.next();
            if !second.is_some() {
                return;
            }
            match first {
                None => (),
                Some("round") => lang.round = second.unwrap().to_string(),
                Some("group") => lang.group = second.unwrap().to_string(),
                Some("scram") => lang.scram = second.unwrap().to_string(),
                Some("result") => lang.result = second.unwrap().to_string(),
                Some("judge") => lang.judge = second.unwrap().to_string(),
                Some("comp") => lang.comp = second.unwrap().to_string(),
                Some("extra attempts") => lang.extra_attempts = second.unwrap().to_string(),
                Some("time limit") => lang.time_limit = second.unwrap().to_string(),
                Some("cumulative limit") => lang.cumulative_limit = second.unwrap().to_string(),
                Some("for") => lang.for_scl = second.unwrap().to_string(),
                Some("and") => lang.and_scl = second.unwrap().to_string(),
                Some("cutoff") => lang.curoff = second.unwrap().to_string(),
                Some("multi") => lang.multi_tl = second.unwrap().to_string(),
                Some("333") => lang.e333 = second.unwrap().to_string(),
                Some("444") => lang.e444 = second.unwrap().to_string(),
                Some("555") => lang.e555 = second.unwrap().to_string(),
                Some("666") => lang.e666 = second.unwrap().to_string(),
                Some("777") => lang.e777 = second.unwrap().to_string(),
                Some("222") => lang.e222 = second.unwrap().to_string(),
                Some("333oh") => lang.e333oh = second.unwrap().to_string(),
                Some("minx") => lang.eminx = second.unwrap().to_string(),
                Some("pyram") => lang.epyram = second.unwrap().to_string(),
                Some("skewb") => lang.eskewb = second.unwrap().to_string(),
                Some("sq1") => lang.esq1 = second.unwrap().to_string(),
                Some("333bf") => lang.e333bf = second.unwrap().to_string(),
                Some("444bf") => lang.e444bf = second.unwrap().to_string(),
                Some("555bf") => lang.e555bf = second.unwrap().to_string(),
                Some("333mbf") => lang.e333mbf = second.unwrap().to_string(),
                Some("clock") => lang.eclock = second.unwrap().to_string(),
                _ => (),
            }
        });
    run(&mut [csv1, csv2, comp].iter().map(|x|x.to_string()), lang);
    Ok(())
}