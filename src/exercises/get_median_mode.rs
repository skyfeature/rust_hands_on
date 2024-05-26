#![allow(dead_code)]

use std::collections::HashMap;


#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn calculate_median(scores: &Vec<i32>) -> i32 {
    let sz = scores.len();

    if sz == 0 {
        return 0;
    }

    if sz % 2 == 1 {
        return scores[sz/2];
    }
    else {
        let left_median = scores[(sz-1)/2];
        let right_median = scores[sz/2];

        return (left_median + right_median)/2;
    }
}

fn calculate_mode(scores: &Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for score in scores {
        let curr_count = counter.entry(*score).or_insert(0);
        *curr_count += 1;
    }

    let mut max_count = 0;
    let mut max_count_score = -1;
    for (score, count) in counter {

        // println!("{}->{}", score, count);
        if count >= max_count {
            max_count = count;
            max_count_score = score;
        }
    }

    return max_count_score;
}

fn find_median_and_mode_of_scores() {
    let mut scores = vec![2, 5, 3, 6, 2, 7, 3, 8, 12, 15, 10, 1, 4, 6, 8, 100, 101, 8, 10, 192, 24, 26];

    scores.sort();

    println!("Median score is: {}", calculate_median(&scores));

    println!("Mode score is: {}", calculate_mode(&scores));
}


fn find_median_and_mode_of_people() {
    let mut people: Vec<Person> = Vec::new();

    let names = ["Ram", "Shyam", "Mohan", "Suresh", "Ramesh", "Mahesh", "Deepak", "Kapil", "Bikash", "Ayush", "Akhil", "Pawan"];
    for year in 30..=41 {
        people.push(Person { name: names[year-30].to_string(), age: year as u32 });
    }

    // println!("{:?}", people);

    // people.sort_by(|a, b| a.age.cmp(b.age));
}


pub fn entry_point() {
    find_median_and_mode_of_scores();

    find_median_and_mode_of_people();
}