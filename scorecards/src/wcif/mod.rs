///Module for creating data structure from json file in wcif format
pub mod json;
pub mod oauth;

use std::collections::HashMap;

use json::*;

use crate::pdf::scorecard::TimeLimit;

pub fn get_rounds(wcif: Wcif) -> Vec<(String, usize)> {
    wcif.events.iter()
        .map(|event|event.rounds
            .iter()
            .map(|round|round.id.to_string()))
        .flatten()
        .map(|str|{
            let mut iter = str.split("-r");
            (iter.next().unwrap().to_string(), usize::from_str_radix(iter.next().unwrap(), 10).unwrap())
        })
        .collect()
}

pub fn get_scorecard_info_for_round(wcif: Wcif, event: &str, round: usize) -> (Vec<usize>, HashMap<usize, String>, TimeLimit, String) {
    let id_map = get_id_map(&wcif.persons);

    //Get advancement
    let activity_id = format!("{}-r{}", event, round - 1);
    let round_json = wcif.events.iter().map(|event|event.rounds.iter()).flatten().find(|round| round.id == activity_id).unwrap();
    let advancement_ids = get_advancement_ids(&round_json, &round_json.advancement_condition);

    //Get time limit
    let activity_id = format!("{}-r{}", event, round);
    let round_json = wcif.events.iter().map(|event|event.rounds.iter()).flatten().find(|round| round.id == activity_id).unwrap();
    let time_limit = match &round_json.time_limit {
        None => TimeLimit::Multi,
        Some(v) => {
            match round_json.cutoff {
                None => match v.cumulative_round_ids.len() {
                    0 => TimeLimit::Single(v.centiseconds),
                    1 => TimeLimit::Cumulative(v.centiseconds),
                    _ => TimeLimit::SharedCumulative(v.centiseconds, v.cumulative_round_ids.clone())
                }
                Some(ref c) => TimeLimit::Cutoff(v.centiseconds, c.attempt_result)
            }
        }
    };
    (advancement_ids, id_map, time_limit, wcif.name)
}

fn get_advancement_amount(round: &Round, advancement_condition: &Option<AdvancementCondition>) -> Option<usize> {
    let number_of_competitors = round.results.len();
    match advancement_condition {
        None => None,
        Some(v) => Some( match v {
            AdvancementCondition::Percent(level) => number_of_competitors * level / 100,
            AdvancementCondition::Ranking(level) => *level,
            AdvancementCondition::AttemptResult(level) => {
                let x = round.results.iter().enumerate().find(|(_, result)|{
                    match result.average {
                        -1 => true,
                        average => average as usize > *level
                    }
                }).map(|(x, _)| x);
                let percent = get_advancement_amount(round, &Some(AdvancementCondition::Percent(75))).unwrap();
                match x {
                    Some(v) if v < percent => v,
                    _ => percent
                }
            }
        })
    }
}

fn get_id_map(persons: &Vec<Person>) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    persons.iter().for_each(|p|if let Some(v) = p.registrant_id { map.insert(v, p.name.clone()); });
    map
}

fn get_advancement_ids(round: &Round, advancement_condition: &Option<AdvancementCondition>) -> Vec<usize> {
    let advancement_amount = get_advancement_amount(round, advancement_condition);
    match advancement_amount {
        None => return vec![],
        Some(advancement_amount) => {
            let filtered = round.results.iter().filter(|result| result.ranking <= advancement_amount).collect::<Vec<_>>();
            if filtered.len() > advancement_amount {
                let not_included = filtered.last().unwrap().ranking;
                return filtered.iter().filter(|result| result.ranking != not_included).map(|result| result.person_id).collect();
            }
            filtered.iter().map(|result| result.person_id).collect()
        }
    }
}