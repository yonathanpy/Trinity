use std::collections::HashMap;
use crate::bounds::{check, Limit};
use crate::journal::Journal;

pub fn evaluate(
    state: &HashMap<String, f64>,
    limits: &HashMap<String, Limit>,
    journal: &mut Journal,
) -> String {
    let faults = check(state, limits);

    if !faults.is_empty() {
        for (k, v) in faults {
            journal.append(format!("FAULT:{}={}", k, v));
        }
        return "FAULT".into();
    }

    "STABLE".into()
}
