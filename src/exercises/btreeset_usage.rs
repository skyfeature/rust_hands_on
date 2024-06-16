#![allow(dead_code)]

use core::num;
use std::env;
use std::fs::File;
use std::io::{self, BufRead,Write};
use std::fs::read_to_string;

use std::collections::{BTreeSet, BTreeMap};

fn get_max_knowledge_fails(d: i32, s: &[i32], e: &[i32], a: &[i32], k: i32) -> i64 {
    /* Not good for large number of meeting days. */
    if k <= 0 {
        return 0;
    }

    println!("K is: {}", k);

    struct MeetingDay(i64, BTreeSet<i32>);

    let mut learning: Vec<MeetingDay> = Vec::new();
    learning.resize_with(d as usize + 1, || MeetingDay(0, BTreeSet::new()));

    println!("Assigned vector of size: {}", d + 1);

    let n = s.len();

    for meeting in 0..n {

        let start_day = s[meeting];
        let end_day = e[meeting];

        let knowledge = a[meeting];

        println!("Meeting num processing: {}", meeting);

        for day in start_day..=end_day {
            let MeetingDay(total_knowledge, knowledge_days) = &mut learning[day as usize];

            if knowledge_days.len() < k as usize {
                *total_knowledge += knowledge as i64;
                knowledge_days.insert(knowledge);
            }
            else {
                if *knowledge_days.first().unwrap() < knowledge {
                    *total_knowledge += knowledge as i64;
                    knowledge_days.insert(knowledge);

                    let smallest_knowledge = knowledge_days.pop_first().unwrap();
                    *total_knowledge -= smallest_knowledge as i64;
                }
            }
        }
    }

    let mut result = 0 as i64;

    for MeetingDay(total_knowledge, _) in &learning {
        result = std::cmp::max(result, *total_knowledge);
    }

    return result;

}

fn get_max_knowledge(_d: i32, s: &[i32], e: &[i32], a: &[i32], k: i32) -> i64 {
    if k <= 0 {
        return 0;
    }
    let mut learning: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    for meeting in 0..s.len() {
        let start_day = s[meeting];
        let end_day = e[meeting];
        let knowledge = a[meeting];

        let val = learning.entry(start_day).or_insert(Vec::new());
        val.push(knowledge);

        let val = learning.entry(end_day+1).or_insert(Vec::new());
        val.push(-1 * knowledge);
    }

    for (_, val) in learning.iter_mut() {
        val.sort_by(|a, b| b.cmp(a));
    }

    let mut result: i64 = 0;
    let mut curr_knowledge: i64 = 0;

    for (_, val) in learning.iter() {

        let mut num_events = 0;
        for knowledge in val.iter() {
            if *knowledge < 0 {
                curr_knowledge += *knowledge as i64;
            }
            else if num_events <= k {
                curr_knowledge += *knowledge as i64;

                num_events += 1;
            }
        }

        result = std::cmp::max(result, curr_knowledge);
    }

    return result;
}

fn fetch_args() -> (i32, Vec<i32>, Vec<i32>, Vec<i32>, i32) {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let d = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let s_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut s: Vec<i32> = Vec::with_capacity(s_count as usize);

    for _ in 0..s_count {
        let s_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        s.push(s_item);
    }

    let e_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut e: Vec<i32> = Vec::with_capacity(e_count as usize);

    for _ in 0..e_count {
        let e_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        e.push(e_item);
    }

    let a_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut a: Vec<i32> = Vec::with_capacity(a_count as usize);

    for _ in 0..a_count {
        let a_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        a.push(a_item);
    }

    let k = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    return (d, s, e, a, k);
}

fn fetch_args_from_file(filename: &str) -> (i32, Vec<i32>, Vec<i32>, Vec<i32>, i32) {

    let fh = read_to_string(filename).unwrap();
    let mut input_lines = fh.lines();

    let d: i32 = input_lines.next().unwrap().trim().parse().unwrap();

    let s_count: usize = input_lines.next().unwrap().trim().parse().unwrap();
    let mut s: Vec<i32> = Vec::with_capacity(s_count);
    for _ in 0..s_count {
        s.push(input_lines.next().unwrap().trim().parse().unwrap());
    }

    let e_count: usize = input_lines.next().unwrap().trim().parse().unwrap();
    let mut e: Vec<i32> = Vec::with_capacity(e_count);
    for _ in 0..e_count {
        e.push(input_lines.next().unwrap().trim().parse().unwrap());
    }

    let a_count: usize = input_lines.next().unwrap().trim().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(a_count);
    for _ in 0..a_count {
        let a_item = input_lines.next().unwrap().trim().parse().unwrap();
        a.push(a_item);
    }

    let k: i32 = input_lines.next().unwrap().trim().parse().unwrap();

    return (d, s, e, a, k);

}

pub fn entry_point() {

    let filename = "src/exercises/INPUT2.txt";

    // let (d, s, e, a, k) = fetch_args();
    let (d, s, e, a, k) = fetch_args_from_file(filename);


    let result = get_max_knowledge(d, &s, &e, &a, k);
    println!("Result: {}", result);

    let mut fptr = File::create("src/exercises/MaxResult.txt").unwrap();
    writeln!(&mut fptr, "{}", result).ok();

}