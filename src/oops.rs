#![allow(dead_code)]

pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, val: i32) {
        self.list.push(val);

        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let val = self.list.pop()?;
        
        self.update_average();
        Some(val)
    }

    pub fn average(&self) -> f64 {
        self.avg
    }

    fn update_average(&mut self) {
        let total_val: i32 = self.list.iter().sum();

        self.avg = (total_val as f64)/(self.list.len() as f64);
    }
}


pub fn entry_point() {
    let mut collect = AveragedCollection {
        list: Vec::new(),
        avg: 0.0,
    };

    collect.add(2);
    collect.add(3);
    collect.add(4);

    println!("Avg is: {}", collect.average());

    collect.add(5);
    println!("Avg is: {}", collect.average());

    collect.remove();
    println!("Avg is: {}", collect.average());
}