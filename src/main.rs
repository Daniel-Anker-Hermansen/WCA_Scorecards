use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::loaders::default::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use printpdf::{PdfDocumentReference, PdfDocument, Mm, Point, Line, LineDashPattern, Color, Greyscale, PdfLayerReference, IndirectFontRef};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::env::args;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Scorecard<'a> {
    event: &'a str,
    round: i8,
    group: i8,
    station: i8,
    id: usize,
}

enum TimeLimit<'a> {
    Single(usize),
    Cumulative(usize),
    SharedCumulative(usize, Vec<&'a str>),
    Cutoff(usize, usize),
    Multi,
    None
}

struct Language {
    round: String,
    group: String,
    scram: String,
    result: String,
    judge: String,
    comp: String,
    extra_attempts: String,
    time_limit: String,
    cumulative_limit: String,
    for_scl: String,
    and_scl: String,
    curoff: String,
    multi_tl: String,
    e333: String,
    e444: String,
    e555: String,
    e666: String,
    e777: String,
    e222: String,
    e333oh: String,
    eclock: String,
    eminx: String,
    epyram: String,
    e333bf: String,
    e444bf: String,
    e555bf: String,
    e333mbf: String,
    esq1: String,
    eskewb: String
}

impl Language {
    fn english() -> Self {
        Language { 
            round: format!("Round"), 
            group: format!("Group"), 
            scram: format!("scram"), 
            result: format!("result"), 
            judge: format!("judge"), 
            comp: format!("comp"), 
            extra_attempts: format!("Extra attempts"),
            time_limit: format!("Time limit"), 
            cumulative_limit: format!("Cumulative limit"), 
            for_scl: format!("for"), 
            and_scl: format!("and"), 
            curoff: format!("Two attempts to get below"), 
            multi_tl: format!("10:00 per cube up to 60:00"), 
            e333: format!("3x3x3 Cube"), 
            e444: format!("4x4x4 Cube"), 
            e555: format!("5x5x5 Cube"), 
            e666: format!("6x6x6 Cube"), 
            e777: format!("7x7x7 Cube"), 
            e222: format!("2x2x2 Cube"), 
            e333oh: format!("3x3x3 Cube One Handed"), 
            eclock: format!("Clock"), 
            eminx: format!("Megaminx"), 
            epyram: format!("Pyraminx"), 
            e333bf: format!("3x3x3 Blindfolded"), 
            e444bf: format!("4x4x4 Blindfolded"), 
            e555bf: format!("5x5x5 Blindfolded"), 
            e333mbf: format!("Multiple Blindfolded"), 
            esq1: format!("Square 1"), 
            eskewb: format!("Skewb")
        }
    }
}

fn main() {
    run(Language::english());
}

fn run(language: Language) {
    let mut args = args();
    let csv = args.nth(1).unwrap();
    let data = match std::fs::read_to_string(csv.clone()) {
        Err(_) => panic!("Could not find csv for groups and stations"),
        Ok(s) => s
    };
    let mut csv_file = data.lines();
    let header = csv_file.next().unwrap().split(",").skip(2);
    let mut map = HashMap::new();
    let mut k = csv_file
        .filter(|x|*x != "")
        .map(|person|{
            let mut iter = person.split(",");
            let name = iter.next().unwrap();
            let id = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();
            map.insert(id, name);
            iter.zip(header.clone()).map(move |(asign, event)|{
                if asign == "" {
                    return (id, event, None, None)
                }
                else {
                    let mut info = asign.split(";");
                    let group = match info.next() {
                        None => return (id, event, None, None),
                        Some(v) => i8::from_str_radix(v, 10).unwrap()
                    };
                    let station = i8::from_str_radix(info.next().unwrap(), 10).unwrap();
                    (id, event, Some(group), Some(station))
                }
            }
        )
    })
        .flatten()
        .filter(|(_, event, v, _)|v.is_some() && *event != "333fm")
        .map(|(id, event, group, station)|{
            Scorecard {
                id,
                group: group.unwrap(),
                round: 1,
                station: station.unwrap(),
                event
            }
        })
        .collect::<Vec<_>>();
    k.sort();

    let limit_csv = args.next().unwrap();
    let limit_data = match std::fs::read_to_string(limit_csv) {
        Err(_) => panic!("Could not find csv for time limits"),
        Ok(s) => s
    };
    let mut limit = limit_data.lines();
    let event_list = limit.next().unwrap().split(",");
    let limit_data = limit.next().unwrap().split(",");

    let mut limits = HashMap::new();
    limit_data.zip(event_list).for_each(|(x, event)|{
        let mut iter = x.split(";");
        let t = iter.next();
        let v = match t {
            None => {
                limits.insert(event, TimeLimit::None);
                return;
            }
            Some(v) => v,
        };
        match v {
            "T" => limits.insert(event, TimeLimit::Single(usize::from_str_radix(iter.next().unwrap(), 10).unwrap())),
            "C" => limits.insert(event, TimeLimit::Cumulative(usize::from_str_radix(iter.next().unwrap(), 10).unwrap())),
            "K" => limits.insert(event, TimeLimit::Cutoff(usize::from_str_radix(iter.next().unwrap(), 10).unwrap(), usize::from_str_radix(iter.next().unwrap(), 10).unwrap())),
            "S" => limits.insert(event, TimeLimit::SharedCumulative(usize::from_str_radix(iter.next().unwrap(), 10).unwrap(), iter.collect::<Vec<_>>())),
            "M" => limits.insert(event, TimeLimit::Multi),
            _ => panic!("Malformatted time limit for event: {}", event)
        };
    });
    let v = args.next().unwrap();
    let doc = scorecards_to_pdf(k, &v, &map, &limits, language);
    doc.save(&mut BufWriter::new(File::create(format!("{}_scorecards.pdf", &csv[..csv.len() - 4])).unwrap())).unwrap();
}

fn scorecards_to_pdf(scorecards: Vec<Scorecard>, competition: &str, map: &HashMap<usize, &str>, limits: &HashMap<&str, TimeLimit>, language: Language) -> PdfDocumentReference {
    let (doc, page, layer) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let mut pages = vec![(page, layer)];
    let mut scorecards: Vec<Option<Scorecard>> = scorecards.into_iter().map(|scorecard|Some(scorecard)).collect();
    while scorecards.len() % 6 != 0 {
        scorecards.push(None);
    }

    let n_pages = (scorecards.len() + 5) / 6;
    scorecards = (0..scorecards.len()).map(|x|{
        let page = x / 6;
        let pos = x % 6;
        scorecards[pos * n_pages + page]
    }).collect::<Vec<_>>();

    let mut scorecard_pages = vec![];
    for i in 0..scorecards.len() / 6 {
        scorecard_pages.push(&scorecards[(i * 6)..(i * 6) + 6])
    }
    for _ in 0..scorecard_pages.len() - 1 {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
        pages.push((page, layer));
    }
    let pages = pages.into_iter().zip(scorecard_pages);
    for ((page, layer), scorecards) in pages {
        let current_layer = doc.get_page(page).get_layer(layer);

        let points1 = vec![(Point::new(Mm(105.0), Mm(0.0)), false),
                        (Point::new(Mm(105.0), Mm(297.0)), false)];

        let line1 = Line {
            points: points1,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        let points2 = vec![(Point::new(Mm(0.0), Mm(99.0)), false),
                        (Point::new(Mm(210.0), Mm(99.0)), false)];

        let line2 = Line {
            points: points2,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        let points3 = vec![(Point::new(Mm(0.0), Mm(198.0)), false),
                        (Point::new(Mm(210.0), Mm(198.0)), false)];

        let line3 = Line {
            points: points3,
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        };

        let width = Some(5);
        let gap = Some(10);
        let dash_pattern = LineDashPattern::new(0, width, gap, width, gap, width, gap);
        let outline_color = Color::Greyscale(Greyscale::new(0.0, None));

        // More advanced graphical options
        current_layer.set_overprint_stroke(true);
        current_layer.set_line_dash_pattern(dash_pattern);
        current_layer.set_outline_color(outline_color);
        current_layer.set_outline_thickness(0.5);
        current_layer.add_shape(line1);
        current_layer.add_shape(line2);
        current_layer.add_shape(line3);

        let font3 = SystemSource::new().select_best_match(&[FamilyName::SansSerif],
            &Properties::new().style(font_kit::properties::Style::Normal))
            .unwrap();

        let font = match &font3 {
            Handle::Path {
                path,
                ..
            } => doc.add_external_font(&File::open(path).unwrap()).unwrap(),
            Handle::Memory {
                ..
            } => panic!("Let's hope it finds the path")
        };

        let font2 = font3.load().unwrap();
        let dash_pattern = LineDashPattern::new(0, None, None, None, None, None, None);
        current_layer.set_line_dash_pattern(dash_pattern);

        for (scorecard, number) in scorecards.into_iter().zip(0..6) {
            match scorecard {
                None => (),
                Some(v) => draw_scorecard(number, v, competition, &current_layer, &font, &font2, map, limits, &language)
            }
        }
    }
    doc
}

fn draw_scorecard(number: i8, Scorecard { id, round, group, station, event }: &Scorecard, competition: &str, current_layer: &PdfLayerReference, font: &IndirectFontRef, font2: &Font, map: &HashMap<usize, &str>, limits: &HashMap<&str, TimeLimit>, language: &Language) {
    let (write_text, draw_square) = get_funcs(number, font2, current_layer, font);
    //Competiton
    write_text(competition, Alignment::Centered, 52.5, 7.0, 10.0);
    write_text(match *event {
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
    }, Alignment::Centered, 52.5, 12.0, 14.0);
    write_text(&format!("{}: {} | {}: {}", language.round, round, language.group, group), Alignment::Centered, 52.5, 16.0, 10.0);
    draw_square(5.0, 19.0, 10.0, 5.5);
    write_text(id.to_string().as_str(), Alignment::Centered, 10.0, 23.0, 10.0);
    draw_square(15.0, 19.0, 85.0, 5.5);
    write_text(map[id], Alignment::Left, 16.0, 23.0, 10.0);

    let attempts_amount = match *event {
        "666" | "777" | "333mbf" | "333bf" | "444bf" | "555bf" => 3,
        _ => 5
    };

    let height = 8.2;
    let distance = 8.2;
    let sign_box_width = 10.0;
    let mut attempts_start_height = 29.5;
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

    attempts_start_height += attempts_amount as f64 * distance + 4.0;
    write_text(&language.extra_attempts, Alignment::Centered, 52.5, attempts_start_height - 1.0, 8.0);
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
        TimeLimit::SharedCumulative(z, vec) => format!("{}: {} {} {}", language.cumulative_limit, time_string(*z), language.for_scl, vec.iter().map(|x|match *x {
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
        }).collect::<Vec<_>>().join(&format!(" {} ", language.and_scl))),
        TimeLimit::Multi => language.multi_tl.to_owned(),
        TimeLimit::None => format!("")
    };

    write_text(&limit, Alignment::Right, 100.0, 94.0, 7.0);
    write_text(station.to_string().as_str(), Alignment::Right, 100.0, 12.0, 25.0);
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

#[inline]
fn get_funcs<'a>(number: i8, font_path: &'a Font, current_layer: &'a PdfLayerReference, font: &'a IndirectFontRef) -> (
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

fn get_width_of_string(font: &Font, string: &str, font_size: f64) -> f64 {
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
    ((width / (upem as f64 / font_size) as f32) / 2.83) as f64
}