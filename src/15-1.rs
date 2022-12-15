use regex::Regex;
use std::fs::read_to_string;

fn distance(x: &(i32, i32), y: &(i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn main() {
    let re = Regex::new(r"x=(-?\d*), y=(-?\d*):.*x=(-?\d*), y=(-?\d*)").unwrap();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split('\n');
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    let mut min_x = 0;
    let mut max_x = 0;
    let mut max_d = 0;
    for i in f {
        for cap in re.captures_iter(i) {
            let sensor = (&cap[1], &cap[2]);
            let beacon = (&cap[3], &cap[4]);
            let sensor = (
                sensor.0.parse::<i32>().unwrap(),
                sensor.1.parse::<i32>().unwrap(),
            );
            let beacon = (
                beacon.0.parse::<i32>().unwrap(),
                beacon.1.parse::<i32>().unwrap(),
            );
            if sensor.0 < min_x {
                min_x = beacon.0;
            }
            if sensor.0 > max_x {
                max_x = beacon.0;
            }
            let d = distance(&sensor, &beacon);
            if d > max_d {
                max_d = d;
            }
            sensors.push((sensor, d));
            beacons.push(beacon);
        }
    }
    let y = 2000000;
    let mut count = 0;
    'outer: for i in (min_x - max_d)..=(max_x + max_d) {
        let p1 = (i, y);
        if beacons.iter().any(|x| (x.0, x.1) == p1) {
            continue;
        }
        for j in &sensors {
            let p2 = (j.0 .0, j.0 .1);
            let d = j.1;
            if distance(&p1, &p2) <= d {
                count += 1;
                continue 'outer;
            }
        }
    }
    println!("{}", count);
}
