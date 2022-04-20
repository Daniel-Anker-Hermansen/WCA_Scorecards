use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Wcif {
    pub events: Vec<Event>
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub rounds: Vec<Round>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub time_limit: serde_json::Value,
    pub cutoff: Option<usize>,
    pub advancement_condition: Option<AdvancementCondition>,
    pub results: Vec<Result>
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
    let p = serde_json::from_str(&json).unwrap();
    p
}