pub fn event_list_to_html(id: &str, rounds: Vec<(String, usize)>) -> String {
    rounds.iter().map(|(eventid, round)|format!("<a href=round/?eventid={e}&round={r} download={n}{e}{r}.pdf>{e}, {r}</a><br>", r = round, e = eventid, n = id)).collect()
}