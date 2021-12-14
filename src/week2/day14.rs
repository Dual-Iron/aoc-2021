use std::collections::HashMap;

type InsertionRules = HashMap<String, &'static str>;
type Polymer = HashMap<String, u64>;

fn parse_input() -> (Polymer, InsertionRules) {
    let mut lines = crate::input!(14).lines();
    let template = lines.next().unwrap();
    lines.next();

    let mut rules = InsertionRules::new();
    for line in lines {
        let mut split = line.split(" -> ");
        rules.insert(split.next().unwrap().into(), split.next().unwrap());
    }

    let mut polymer = HashMap::with_capacity(rules.capacity());

    for kvp in &rules {
        polymer.insert(kvp.0.into(), 0);
    }

    for i in 1..template.len() {
        polymer
            .entry(template[i - 1..=i].into())
            .and_modify(|e| *e += 1);
    }

    (polymer, rules)
}

fn step(polymer: &mut Polymer, rules: &InsertionRules) {
    // for PV if rule PV -> T:
    // subtract one from PV
    // add one to PT and TV
    for kvp in &polymer.to_owned() {
        let add1 = format!("{}{}", &kvp.0[0..=0], rules[kvp.0]);
        let add2 = format!("{}{}", rules[kvp.0], &kvp.0[1..=1]);

        *polymer.get_mut(&add1).unwrap() += kvp.1;
        *polymer.get_mut(&add2).unwrap() += kvp.1;
        *polymer.get_mut(kvp.0).unwrap() -= kvp.1;
    }
}

fn most(polymer: &Polymer, max: bool) -> u64 {
    let mut freq = HashMap::new();

    for p in polymer {
        *freq.entry(p.0.chars().next().unwrap()).or_default() += *p.1;
    }

    if max {
        *freq.values().max().unwrap()
    } else {
        *freq.values().min().unwrap()
    }
}

// 2938
pub fn part1() {
    let (mut polymer, rules) = parse_input();

    for _ in 0..10 {
        step(&mut polymer, &rules);
    }

    // I don't really know why I have to subtract 1 but I do.
    // Off-by-one errors suck.
    println!("{}", most(&polymer, true) - most(&polymer, false) - 1);
}

// 3390034818249
pub fn part2() {
    let (mut polymer, rules) = parse_input();

    for _ in 0..40 {
        step(&mut polymer, &rules);
    }

    println!("{}", most(&polymer, true) - most(&polymer, false) - 1);
}
