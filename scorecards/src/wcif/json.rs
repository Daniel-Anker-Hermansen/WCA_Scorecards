use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Wcif {
    pub name: String,
    pub events: Vec<Event>,
    pub persons: Vec<Person>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub name: String,
    pub registrant_id: Option<usize>
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub rounds: Vec<Round>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub id: String,
    pub time_limit: Option<TimeLimit>,
    pub cutoff: Option<Cutoff>,
    pub advancement_condition: Option<AdvancementCondition>,
    pub results: Vec<Result>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeLimit {
    pub centiseconds: usize,
    pub cumulative_round_ids: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cutoff {
    pub number_of_attempts: usize,
    pub attempt_result: usize
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "level", rename_all = "camelCase")]
pub enum AdvancementCondition {
    Percent(usize),
    Ranking(usize),
    AttemptResult(usize)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub person_id: usize,
    pub ranking: usize,
    pub average: isize
}

pub fn parse(json: String) -> Wcif {
    serde_json::from_str(&json).unwrap()
}