use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use crate::language::Language;
use scorecard::{Scorecard, TimeLimit, scorecards_to_pdf};

mod scorecard;

pub fn run(args: &mut std::slice::Iter<'_, &str>, language: Language) {
    let csv = args.next().unwrap();
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
    let doc = scorecards_to_pdf(k, match args.next() {
        None => "No competion name given",
        Some(v) => v
    }, &map, &limits, language);
    doc.save(&mut BufWriter::new(File::create(format!("{}_scorecards.pdf", &csv[..csv.len() - 4])).unwrap())).unwrap();
}