#![allow(dead_code)]

use std::collections::HashMap;

fn hashmap_initialization() {
    let mut scores = HashMap::new();

    scores.insert("KKR".to_string(), 30u32);
    scores.insert("PKB".to_string(), 50);

    for (team, score) in &mut scores {
        println!("key: {}, val: {}", team, score);
        *score *= 2;
    }

    println!("{:?}", scores);

    let team_srh = String::from("SRH");
    let score_srh = scores.get(&team_srh).copied().unwrap_or(0);

    println!("Score for {} team is: {}", team_srh, score_srh);

    scores.insert("CSK".to_string(), 50);
    scores.insert("CSK".to_string(), 100); // updated the value.

    println!("{:?}", scores);

    scores.entry("RCB".to_string()).or_insert(22);
    scores.entry("KKR".to_string()).or_insert(250); // inserts only if the key isn't present already

    println!("{:?}", scores);

    let winners: Vec<String> = vec!["PKB".to_string(), "MI".to_string()];

    for team in &winners {
        let old_score = scores.entry(team.clone()).or_insert(0);
        *old_score += 111;
    }

    println!("{:?}", winners);
    println!("{:?}", scores);


}



pub fn entry_point() {
    hashmap_initialization();
}