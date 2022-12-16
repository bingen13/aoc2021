use regex::Regex;
use std::fs::read_to_string;

fn distance(x: &(i64, i64), y: &(i64, i64)) -> i64 {
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
                sensor.0.parse::<i64>().unwrap(),
                sensor.1.parse::<i64>().unwrap(),
            );
            let beacon = (
                beacon.0.parse::<i64>().unwrap(),
                beacon.1.parse::<i64>().unwrap(),
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
    let mut points = Vec::new();
    for s in &sensors {
        let (x, y) = (s.0 .0, s.0 .1);
        let d = s.1;
        for i in 0..d {
            points.push((x + (d - i) + 1, y + i));
            points.push((x - i, y + (d - i) + 1));
            points.push((x - ((d - i) + 1), y - i));
            points.push((x + i, y - ((d - i) + 1)));
        }
    }

    for p in points {
        if (p.0 >= 0) && (p.0 <= 4000000) && (p.1 >= 0) && (p.1 <= 4000000) {
            let mut found = true;
            for s in &sensors {
                let p1 = s.0;
                let d = s.1;
                if distance(&p, &p1) <= d {
                    /*
                                        println!(
                                            "Point {:?} is too close to {:?}. {} of {}.",
                                            p,
                                            p1,
                                            distance(&p, &p1),
                                            d
                                        );
                    */
                    found = false;
                }
            }
            if found {
                println!("{:?}, {}.", p, 4000000 * p.0 + p.1);
            }
        }
    }
}
