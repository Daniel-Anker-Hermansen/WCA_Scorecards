use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::loaders::default::Font;
use font_kit::properties::{Properties, Style, Weight};
use font_kit::source::SystemSource;
use printpdf::{IndirectFontRef, PdfDocumentReference};
use std::fs::File;

pub type FontWidth = Font;
pub type FontPDF = IndirectFontRef;

pub fn load_fonts(doc: &PdfDocumentReference, family: FamilyName, weight: Weight) -> (FontWidth, FontPDF) {
    let handle = SystemSource::new().select_best_match(&[family],
        &Properties::new().style(Style::Normal).weight(weight))
        .unwrap();

    let font = match &handle {
        Handle::Path {
            path,
            ..
        } => doc.add_external_font(&File::open(path).unwrap()).unwrap(),
        Handle::Memory {
            ..
        } => panic!("Let's hope it finds the path")
    };

    let font_width = handle.load().unwrap();

    (font_width, font)
}