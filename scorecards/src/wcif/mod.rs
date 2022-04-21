///Module for creating data structure from json file in wcif format
pub mod json;
mod oauth;

use json::*;

pub fn get_advancement(id: &str) -> () {
    let json = oauth::get_wcif(id);

    let wcif = json::parse(json);  

    wcif.events.iter().for_each(|event| {
        event.rounds.iter().for_each(|round| {
            let s = get_advancement_amount(&round, &round.advancement_condition);
            println!("{:?}", s);   
        })
    })
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