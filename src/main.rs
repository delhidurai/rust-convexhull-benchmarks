// use std::fs::File;
// use std::io::prelude::*;

extern crate time;
use time::PreciseTime;

use std::time::{Instant, Duration};

extern crate rustalgo;
// use rustalgo::inputset_gen::*;
use rustalgo::sample_data::*;
use rustalgo::points::*;
use rustalgo::convexhull::*;

fn main() {
    // let mut file = File::create("sample.txt").unwrap();
    // let number_of_vertex = 3;
    // let input_set_10 = get_input_set(10, number_of_vertex);
    // let mut output = String::new();
    // for point in input_set_10 {
    //     output += &format!("({},{}),", point.x, point.y);
    // }
    // file.write_all(output.as_bytes()).unwrap();
    let mut input_set_10: Vec<Point2D> = triangle_10().iter().map(|p| Point2D::new(p.0,p.1)).collect();
    benchmark_convex_hull_algorithms(&mut input_set_10)
}

/// Benchmarks all the 3 algorithms for same input
/// The output is printed to the console
fn benchmark_convex_hull_algorithms(input_set: &mut Vec<Point2D>) {
    //graham scan algorithm
    let now = Instant::now();
    graham_scan(input_set);
    let elapsed = now.elapsed();
    let time = Time::new(elapsed);
    println!("graham_scan: {:?} s", time.seconds());
    //jarvis march algorithm
    let now = Instant::now();
    jarvis_march(input_set);
    let elapsed = now.elapsed();
    let time = Time::new(elapsed);
    println!("jarvis_march: {} s", time.seconds());
    //chans algorithm
    // let now = Instant::now();
    // chans_algorithm(input_set);
    // let elapsed = now.elapsed();
    // let time = Time::new(elapsed);
    // println!("chans_algorithm: {} s", time.seconds());
}

/// Computes the duration in various
/// time units
#[derive(Debug, Copy, Clone)]
struct Time {
    seconds: f64,
    milli_seconds: f64,
    nano_seconds: f64,
}

/// implementation for accessing time duration
/// in various time units
impl Time {
    fn new(duration: Duration) -> Time {
        let sec = (duration.as_secs() as f64) + (duration.subsec_nanos() as f64 / 1000_000_000.0);

        Time {
            seconds: sec,
            milli_seconds: 0.0,
            nano_seconds: 0.0
        }
    }

    fn seconds(&self) -> f64 {
        self.seconds
    }
    
    fn milli_seconds(&self) -> f64 {
        self.milli_seconds
    }
    
    fn nano_seconds(&self) -> f64 {
        self.nano_seconds
    }
}