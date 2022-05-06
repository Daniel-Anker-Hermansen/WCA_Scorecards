use font_kit::{family_name::FamilyName, properties::Weight};
use printpdf::{PdfDocumentReference, PdfDocument, Mm, Point, Line, LineDashPattern, Color, Greyscale, PdfLayerReference};
use std::{collections::HashMap};
use crate::language::Language;
use super::font::{load_fonts, FontWidth, FontPDF};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scorecard<'a> {
    pub event: &'a str,
    pub round: i8,
    pub group: i8,
    pub station: i8,
    pub id: usize,
}

pub enum TimeLimit {
    Single(usize),
    Cumulative(usize),
    SharedCumulative(usize, Vec<String>),
    Cutoff(usize, usize),
    Multi,
    None
}

pub fn scorecards_to_pdf(scorecards: Vec<Scorecard>, competition: &str, map: &HashMap<usize, String>, limits: &HashMap<&str, TimeLimit>, language: Language) -> PdfDocumentReference {
    let (doc, page, layer) = PdfDocument::new(competition, Mm(210.0), Mm(297.0), "Layer 1");
    let mut pages = vec![(page, layer)];
    let mut scorecards: Vec<Option<Scorecard>> = scorecards.into_iter().map(|scorecard|Some(scorecard)).collect();
    while scorecards.len() % 6 != 0 {
        scorecards.push(None);
    }

    let n_pages = scorecards.len() / 6;
    scorecards = (0..scorecards.len()).map(|x|{
        let page = x / 6;
        let pos = x % 6;
        scorecards[pos * n_pages + page]
    }).collect::<Vec<Option<Scorecard>>>();

    let mut scorecard_pages = vec![];
    for i in 0..n_pages {
        scorecard_pages.push(&scorecards[(i * 6)..(i * 6) + 6])
    }
    for _ in 1..scorecard_pages.len() {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
        pages.push((page, layer));
    }
    let pages = pages.into_iter().zip(scorecard_pages);
    for ((page, layer), scorecards) in pages {
        let current_layer = doc.get_page(page).get_layer(layer);
        let points1 = vec![(Point::new(Mm(105.0), Mm(0.0)), false),
                        (Point::new(Mm(105.0), Mm(297.0)), false)];
        let points2 = vec![(Point::new(Mm(0.0), Mm(99.0)), false),
                        (Point::new(Mm(210.0), Mm(99.0)), false)];
        let points3 = vec![(Point::new(Mm(0.0), Mm(198.0)), false),
                        (Point::new(Mm(210.0), Mm(198.0)), false)];
        let line1 = line_from_points(points1);
        let line2 = line_from_points(points2);
        let line3 = line_from_points(points3);
        let width = Some(5);
        let gap = Some(10);
        let dash_pattern = LineDashPattern::new(0, width, gap, width, gap, width, gap);
        let outline_color = Color::Greyscale(Greyscale::new(0.0, None));
        current_layer.set_overprint_stroke(true);
        current_layer.set_line_dash_pattern(dash_pattern);
        current_layer.set_outline_color(outline_color);
        current_layer.set_outline_thickness(0.5);
        current_layer.add_shape(line1);
        current_layer.add_shape(line2);
        current_layer.add_shape(line3);

        let (font_width, font) = load_fonts(&doc, FamilyName::SansSerif, Weight::NORMAL);
        let (font_width_bold, font_bold) = load_fonts(&doc, FamilyName::SansSerif, Weight::BOLD);
        
        let dash_pattern = LineDashPattern::new(0, None, None, None, None, None, None);
        current_layer.set_line_dash_pattern(dash_pattern);

        for (scorecard, number) in scorecards.into_iter().zip(0..6) {
            match scorecard {
                None => (),
                Some(v) => draw_scorecard(number, v, competition, &current_layer, &font, &font_width, &font_bold, &font_width_bold, map, limits, &language)
            }
        }
    }
    doc
}

fn line_from_points(points: Vec<(Point, bool)>) -> Line {
    Line {
        points: points,
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    }
}

fn draw_scorecard(number: i8, Scorecard { id, round, group, station, event }: &Scorecard, competition: &str, current_layer: &PdfLayerReference, font: &FontPDF, font2: &FontWidth,  font_bold: &FontPDF, font2_bold: &FontWidth, map: &HashMap<usize, String>, limits: &HashMap<&str, TimeLimit>, language: &Language) {
    let (write_text, draw_square) = get_funcs(number, font2, current_layer, font);
    let (write_bold_text, _) = get_funcs(number, font2_bold, current_layer, font_bold);
    let get_event = get_event_func(language);
    //Competiton
    write_text(competition, Alignment::Centered, 52.5, 7.0, 10.0);
    let (round_text, event_text, group_text) = (format!("{}: {} | ", language.round, round), format!("{}", get_event(event)), format!(" | {}: {}", language.group, group));
    let (round_width, event_width, group_width) = (get_width_of_string(font2, &round_text, 10.0), get_width_of_string(font2_bold, &event_text, 10.0), get_width_of_string(font2, &group_text, 10.0));
    write_text(&round_text, Alignment::Left, 52.5 - (round_width + event_width + group_width) / 2.0, 11.5, 10.0);
    write_bold_text(&event_text, Alignment::Left, 52.5 - (- round_width + event_width + group_width) / 2.0, 11.5, 10.0);
    write_text(&group_text, Alignment::Left, 52.5 - (- round_width - event_width + group_width) / 2.0, 11.5, 10.0);
    draw_square(5.0, 15.0, 10.0, 5.5);
    write_text(id.to_string().as_str(), Alignment::Centered, 10.0, 19.0, 10.0);
    draw_square(15.0, 15.0, 85.0, 5.5);
    write_text(&map[id], Alignment::Left, 16.0, 19.0, 10.0);

    let attempts_amount = match *event {
        "666" | "777" | "333mbf" | "333bf" | "444bf" | "555bf" => 3,
        _ => 5
    };

    let height = 8.2;
    let distance = 8.8;
    let sign_box_width = 10.0;
    let mut attempts_start_height = 25.5;
    write_text(&language.scram, Alignment::Centered, 9.0 + sign_box_width / 2.0, attempts_start_height - 1.0, 7.0);
    write_text(&language.result, Alignment::Centered, (12.0 + 97.0 - sign_box_width) / 2.0, attempts_start_height - 1.0, 7.0);
    write_text(&language.judge, Alignment::Centered, 100.0 - sign_box_width - (sign_box_width / 2.0), attempts_start_height - 1.0, 7.0);
    write_text(&language.comp, Alignment::Centered, 100.0 - (sign_box_width / 2.0), attempts_start_height - 1.0, 7.0);
    for i in 0..attempts_amount {
        let j = i as f64;
        draw_square(9.0, attempts_start_height + j * distance, sign_box_width, height);
        write_text((i + 1).to_string().as_str(), Alignment::Left, 5.0, attempts_start_height - 2.0 + j * distance + height, 12.0);
        draw_square(9.0 + sign_box_width, attempts_start_height + j * distance, 91.0 - 3.0 * sign_box_width, height);
        draw_square(100.0 - 2.0 * sign_box_width, attempts_start_height + j * distance, sign_box_width, height);
        draw_square(100.0 - sign_box_width, attempts_start_height + j * distance, sign_box_width, height);
    }

    attempts_start_height += attempts_amount as f64 * distance + 3.8;
    write_text(&language.extra_attempts, Alignment::Centered, 52.5, attempts_start_height - 1.0, 7.0);
    for i in 0..2 {
        let j = i as f64;
        draw_square(9.0, attempts_start_height + j * distance, sign_box_width, height);
        write_text("_", Alignment::Left, 5.0, attempts_start_height - 2.0 + j * distance + height, 12.0);
        draw_square(9.0 + sign_box_width, attempts_start_height + j * distance, 91.0 - 3.0 * sign_box_width, height);
        draw_square(100.0 - 2.0 * sign_box_width, attempts_start_height + j * distance, sign_box_width, height);
        draw_square(100.0 - sign_box_width, attempts_start_height + j * distance, sign_box_width, height);
    }

    let limit = match &limits[event.clone()] {
        TimeLimit::Single(z) => format!("{}: {}", language.time_limit, time_string(*z)),
        TimeLimit::Cumulative(z) => format!("{}: {}", language.cumulative_limit, time_string(*z)),
        TimeLimit::Cutoff(x, z) => format!("{}: {}, {}: {}", language.curoff, time_string(*x), language.time_limit, time_string(*z)),
        TimeLimit::SharedCumulative(z, vec) => format!("{}: {} {} {}", language.cumulative_limit, time_string(*z), language.for_scl, vec.iter().map(|x|get_event(x)).collect::<Vec<_>>().join(&format!(" {} ", language.and_scl))),
        TimeLimit::Multi => language.multi_tl.to_owned(),
        TimeLimit::None => format!("")
    };

    write_text(&limit, Alignment::Right, 100.0, 94.0, 7.0);
    write_bold_text(station.to_string().as_str(), Alignment::Right, 100.0, 12.0, 25.0);
}

fn time_string(mut z: usize) -> String {
    if z >= 6000 {
        let minutes = z / 6000;
        let res = format!("{}:", minutes);
        z = z % 6000;
        format!("{}{:02}.{:02}", res, z / 100, z % 100)
    } else {
        format!("{}.{:02}", z / 100, z % 100)
    }
}

enum Alignment {
    Left,
    Centered,
    Right
}

fn get_funcs<'a>(number: i8, font_path: &'a FontWidth, current_layer: &'a PdfLayerReference, font: &'a FontPDF) -> (
    Box<dyn 'a + Fn(&str, Alignment, f64, f64, f64)>,
    Box<dyn 'a + Fn(f64, f64, f64, f64)>) {
    let (x, y) = match number {
        0 => (0.0, 297.0),
        1 => (105.0, 297.0),
        2 => (0.0, 198.0),
        3 => (105.0, 198.0),
        4 => (0.0, 99.0),
        5 => (105.0, 99.0),
        _ => panic!("My code is not working.")
    };
    (Box::new(move |text, alignment, x1, y1, font_size|{
        current_layer.begin_text_section();
            current_layer.set_font(font, font_size);
            current_layer.set_text_cursor(Mm(match alignment {
                Alignment::Left => x + x1,
                Alignment::Centered => x + x1 - (get_width_of_string(font_path ,text, font_size) / 2.0),
                Alignment::Right => x + x1 - get_width_of_string(font_path ,text, font_size)
            }), Mm(y - y1));
            current_layer.set_line_height(12.0);
            current_layer.write_text(text, font);
        current_layer.end_text_section();
    }),
    Box::new(move |x1, y1, width, height|{
        let points = vec![(Point::new(Mm(x + x1), Mm(y - y1)), false),
        (Point::new(Mm(x + x1 + width), Mm(y - y1)), false),
        (Point::new(Mm(x + x1 + width), Mm(y - y1 - height)), false),
        (Point::new(Mm(x + x1), Mm(y - y1 - height)), false)];
        let square = Line {
            points: points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };
        current_layer.add_shape(square);
    }))
}

fn get_event_func<'a>(language: &'a Language) -> Box<dyn 'a + Fn(&str) -> &'a str> {
    Box::new(move |x|match x {
        "333" => &language.e333,
        "444" => &language.e444,
        "555" => &language.e555,
        "666" => &language.e666,
        "777" => &language.e777,
        "222" => &language.e222,
        "333oh" => &language.e333oh,
        "333fm" => "Filter out FMC",
        "333bf" => &language.e333bf,
        "pyram" => &language.epyram,
        "333mbf" => &language.e333mbf,
        "minx" => &language.eminx,
        "clock" => &language.eclock,
        "444bf" => &language.e444bf,
        "555bf" => &language.e555bf,
        "skewb" => &language.eskewb,
        "sq1" => &language.esq1,
        _ => "Please fix your csv"
    })
}

pub fn get_width_of_string(font: &FontWidth, string: &str, font_size: f64) -> f64 {
    let upem = font.metrics().units_per_em;
    let mut width = 0.0;
    for char in string.chars() {
        if !char.is_whitespace() {
            let id = font.glyph_for_char(char).unwrap();
            let glyph_width = font.advance(id).unwrap().x();
            width += glyph_width

        } else {
            width += upem as f32 / 4.0;
        }
    }
    (width as f64 / (upem as f64 / font_size)) / 2.83
}