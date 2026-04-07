use std::collections::HashMap;

pub struct Limit {
    pub min: f64,
    pub max: f64,
}

pub fn check(
    state: &HashMap<String, f64>,
    limits: &HashMap<String, Limit>,
) -> Vec<(String, f64)> {
    let mut faults = Vec::new();

    for (k, lim) in limits {
        if let Some(v) = state.get(k) {
            if *v < lim.min || *v > lim.max {
                faults.push((k.clone(), *v));
            }
        }
    }

    faults
}
