use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    a: i64,
    b: i64,
}
#[derive(Clone, Copy)]
struct Race {
    time: i64,
    distance: i64,
}
impl Race {
    fn winning_range(self) -> Range {
        let delta = ((self.time.pow(2) - 4 * self.distance) as f64).sqrt();
        dbg!(&delta);
        let a = (-0.5 * (delta - (self.time as f64))) as i64;
        let b = (-0.5 * (-1.0 * delta - (self.time as f64))) as i64;

        Range { a, b }
    }
}

fn main() {
    let times = [44, 70, 70, 80];
    let distances = [283, 1134, 1134, 1491];
    let mut i = 0;
    let mut races: Vec<Race> = Vec::new();
    while i < distances.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
        i += 1;
    }
    let races = races;
    let ranges: Vec<i64> = races
        .par_iter()
        .map(|race| {
            let range = race.winning_range();
            return range.b - range.a;
        })
        .collect();
    let mut result = 1;
    for r in ranges {
        result *= r;
    }
    println!("Hello, world!");
    dbg!(result);
    let long_race = Race {
        time: 44707080,
        distance: 283113411341491,
    };
    let long_race_result = long_race.winning_range().b - long_race.winning_range().a;
    dbg!(long_race_result);
}
