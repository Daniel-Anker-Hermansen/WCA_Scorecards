use wca_scorecards_lib::*;
use std::env::args;

fn main() {
    let args = args();
    let mut stages = None;
    let mut r1 = None;
    let mut subseq = None;
    let mut iter = args.skip(1).peekable();
    while let Some(command) = iter.next() {
        match command.as_str() {
            "--r1" if r1.is_none() => r1 = Some((iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())),
            "--r1" if r1.is_some() => panic!("Specified --r1 twice"),
            "--subseq" if subseq.is_none() => subseq = Some(iter.next().unwrap()),
            "--subseq" if subseq.is_some() => panic!("Specified --subseq twice"),
            "--stages" if stages.is_none() => {
                let mut inner_stages = Stages::new();
                while let Some(v) = iter.peek() {
                    if !v.contains("--") {
                        let v = iter.next().unwrap();
                        let inner_iter = v.split("-").collect::<Vec<_>>();
                        if inner_iter.len() == 1 {
                            inner_stages.add_stage(None, inner_iter[0].parse().unwrap());
                        }
                        else {
                            inner_stages.add_stage(Some(inner_iter[0].chars().next().unwrap()), inner_iter[1].parse().unwrap());
                        }

                    }
                    else {
                        break;                        
                    }
                }
                stages = Some(inner_stages);
            }
            "--stages" if stages.is_some() => panic!("Cannot specify stages twice"),
            _ => panic!("Invalid command given"),
        }
    }

    if r1.is_some() && subseq.is_some() {
        panic!("--r1 and --subseq are mutually exclusive");
    }
    else if let Some((group_csv, limit_csv, competition)) = r1 {
        print_round_1_english(&group_csv, &limit_csv, &competition, stages);
    }
    else if let Some(id) = subseq {
        print_subsequent_rounds(id, stages);
    }
}