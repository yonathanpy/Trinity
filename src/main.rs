mod state;
mod bounds;
mod reactor;
mod journal;

use std::collections::HashMap;
use bounds::Limit;
use state::State;
use journal::Journal;
use reactor::evaluate;

fn main() {
    let mut state = State::new();
    let mut journal = Journal::new();

    let mut limits = HashMap::new();
    limits.insert("temp".into(), Limit { min: 0.0, max: 1000.0 });
    limits.insert("pressure".into(), Limit { min: 0.0, max: 600.0 });
    limits.insert("flux".into(), Limit { min: 0.0, max: 800.0 });

    let mut sample1 = HashMap::new();
    sample1.insert("temp".into(), 400.0);
    sample1.insert("pressure".into(), 200.0);
    sample1.insert("flux".into(), 300.0);

    let mut sample2 = HashMap::new();
    sample2.insert("temp".into(), 1200.0);
    sample2.insert("pressure".into(), 700.0);
    sample2.insert("flux".into(), 900.0);

    let stream = vec![sample1, sample2];

    for pkt in stream {
        state.apply(pkt);
        let status = evaluate(&state.data, &limits, &mut journal);
        journal.append(format!("STATE:{}", status));
    }
}
