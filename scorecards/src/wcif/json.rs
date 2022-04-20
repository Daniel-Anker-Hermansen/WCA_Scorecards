use std::collections::HashMap;

use json::parse;
use json::JsonValue;

use super::{ActivityId, PersonId};

pub fn parse_rounds(json: String) -> HashMap<ActivityId, Vec<PersonId>> {

    let mut json = parse_wcif(json);

    let (mut activity_map, mut event_map) = parse_schedule(json.remove("schedule")); 

    println!("{:?}", activity_map);

    println!("{:?}", event_map);

    unimplemented!();
}

fn parse_wcif(wcif: String) -> JsonValue {
    parse(&wcif).expect("Faulty JSON format")
}

fn parse_schedule(mut json: JsonValue) -> (HashMap<ActivityId, ActivityId>, HashMap<ActivityId, String>) {
    let mut venues = json.remove("venues");
    let mut activity_map = HashMap::new();
    let mut event_map = HashMap::new();
    loop {
        let venue = venues.array_remove(0);

        let (venue_map, eventr_map) = match venue {
            JsonValue::Null => break,
            _ => parse_venue(venue)
        };
        venue_map.into_iter().for_each(|(x,y)|{ activity_map.insert(x, y); });
        eventr_map.into_iter().for_each(|(x,y)|{ event_map.insert(x, y); });
    }
    (activity_map, event_map)
}

fn parse_venue(mut venue: JsonValue) -> (HashMap<ActivityId, ActivityId>, HashMap<ActivityId, String>) {
    let mut rooms = venue.remove("rooms");
    let mut venue_map = HashMap::new();
    let mut event_map = HashMap::new();
    loop {
        let mut room = rooms.array_remove(0);
        let (room_map, eventr_map) = match room {
            JsonValue::Null => break,
            _ => parse_activities(room.remove("activities"))
        };
        room_map.into_iter().for_each(|(x,y)|{ venue_map.insert(x, y); });
        eventr_map.into_iter().for_each(|(x,y)|{ event_map.insert(x, y); });
    }
    (venue_map, event_map)
}

fn parse_activities(mut activities: JsonValue) -> (HashMap<ActivityId, ActivityId>, HashMap<ActivityId, String>) {
    let mut event_map = HashMap::new();
    let mut activity_map = HashMap::new();
    loop {
        let mut activity = activities.array_remove(0);
        match activity {
            JsonValue::Null => break,
            _ => {
                let id = activity["id"].as_u64().unwrap() as usize;
                let activity_code = activity.remove("activityCode");
                event_map.insert(id, activity_code.to_string());
                activity_map.insert(id, id);
                let mut child_activities = activity.remove("childActivities");
                loop {
                    let child_activity = child_activities.array_remove(0);
                    match child_activity {
                        JsonValue::Null => break,
                        _ => {
                            let child_id = child_activity["id"].as_u64().unwrap() as usize;
                            activity_map.insert(child_id, id);
                        }
                    }
                }
            }
        }
    }
    (activity_map, event_map)
}