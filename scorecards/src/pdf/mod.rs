use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use crate::{language::Language, wcif::json::Wcif};
use scorecard::{Scorecard, TimeLimit, scorecards_to_pdf};

pub mod scorecard;
mod font;

pub fn run<I>(args: &mut I, language: Language) where I: Iterator<Item = String> {
    let csv = args.next().unwrap();
    let data = match std::fs::read_to_string(csv.clone()) {
        Err(e) => panic!("Could not find csv for groups and stations: {}", e),
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
            map.insert(id, name.to_string());
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
            "S" => limits.insert(event, TimeLimit::SharedCumulative(usize::from_str_radix(iter.next().unwrap(), 10).unwrap(), iter.map(|x|x.to_string()).collect::<Vec<_>>())),
            "M" => limits.insert(event, TimeLimit::Multi),
            _ => panic!("Malformatted time limit for event: {}", event)
        };
    });
    let mut competition_option = args.next();
    let competition = match competition_option {
        None => "No competion name given",
        Some(ref mut v) => v.as_str()
    };
    let doc = scorecards_to_pdf(k, competition, &map, &limits, language);
    doc.save(&mut BufWriter::new(File::create(competition.split_ascii_whitespace().collect::<String>() + ".pdf").unwrap())).unwrap();
}

pub fn run_from_wcif(wcif: Wcif, event: &str, round: usize, max_group_size: usize) -> Vec<u8> {
    let (competitors, map, limit, competition) = super::wcif::get_scorecard_info_for_round(wcif, event, round);

    let mut limits = HashMap::new();
    limits.insert(event, limit);

    let number_of_groups = (competitors.len() + max_group_size - 1) / max_group_size;
    let group_size = competitors.len() / number_of_groups;
    let modulo = competitors.len() % number_of_groups;
    let mut curr_group = 1;
    let mut curr_station = 0;

    let s = competitors.iter()
        .map(|id| {
            let this_group_size = group_size + match modulo >= curr_group {
                true => 1,
                false => 0
            };
            curr_station += 1;
            if curr_station > this_group_size {
                curr_station = 1;
                curr_group += 1;
            }
            (*id, curr_group, curr_station)
        });
    let k = s.map(|(id, group, station)|{
        Scorecard {
            event,
            round: round as i8,
            group: group as i8,
            station: station as i8,
            id: id
        }
    }).collect::<Vec<_>>();
    
    let doc = scorecards_to_pdf(k, &competition, &map, &limits, Language::english());
    doc.save_to_bytes().unwrap()
}