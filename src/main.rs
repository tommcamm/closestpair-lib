use std::{error::Error, io};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn brute_force(points: &[Point]) -> (Point, Point, f64) {
    let mut min_distance = f64::MAX;
    let mut closest_pair = (points[0], points[1]);
    let n = points.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance(&points[j]);
            if dist < min_distance {
                min_distance = dist;
                closest_pair = (points[i], points[j]);
            }
        }
    }

    (closest_pair.0, closest_pair.1, min_distance)
}


fn closest_pair_recursive(px: &[Point], py: &[Point]) -> (Point, Point, f64) {
    let n = px.len();
    if n <= 3 {
        return brute_force(px);
    }

    let mid = n / 2;
    let mid_point = px[mid];

    let (lx, rx): (&[Point], &[Point]) = px.split_at(mid);
    let mut ly = Vec::with_capacity(mid);
    let mut ry = Vec::with_capacity(n - mid);

    for &point in py.iter() {
        if point.x <= mid_point.x {
            ly.push(point);
        } else {
            ry.push(point);
        }
    }

    let (l1, r1, dl) = closest_pair_recursive(&lx, &ly);
    let (l2, r2, dr) = closest_pair_recursive(&rx, &ry);
    let mut d = dl.min(dr);
    let mut closest_pair = if dl < dr { (l1, r1) } else { (l2, r2) };

    let mut strip = Vec::new();
    for &point in py.iter() {
        if (point.x - mid_point.x).abs() < d {
            strip.push(point);
        }
    }

    for i in 0..strip.len() {
        for j in (i + 1)..strip.len() {
            if (strip[j].y - strip[i].y) >= d {
                break;
            }
            let dist = strip[i].distance(&strip[j]);
            if dist < d {
                d = dist;
                closest_pair = (strip[i], strip[j]);
            }
        }
    }

    (closest_pair.0, closest_pair.1, d)
}

fn closest_pair(points: &[Point]) -> (Point, Point, f64) {
    let mut px: Vec<Point> = points.to_vec();
    let mut py: Vec<Point> = points.to_vec();

    // filter by coordinate
    px.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    py.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    closest_pair_recursive(&px, &py)
}

fn main() -> Result<(), Box<dyn Error>>{

    let stdin = io::stdin();
    let mut test_cases :Vec<Vec<Point>> = Vec::new();

    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        let n :i32 = buf.trim().parse().unwrap();

        // Exit condition
        if n == 0 {
            break;
        }

        let mut points :Vec<Point> = Vec::new();
        for _ in 0..n {
            let mut buf = String::new();
            stdin.read_line(&mut buf).unwrap();

            let mut splitted_input = buf.split_ascii_whitespace();
            points.push(Point { x: splitted_input
                .next()
                .unwrap()
                .parse()
                .unwrap()
                , y: splitted_input.next()
                .unwrap()
                .parse()
                .unwrap()
             });
        }

        test_cases.push(points);
    }

    for test in test_cases {
        let (p1, p2, _) = closest_pair(&test);
        println!("{} {} {} {}", p1.x, p1.y, p2.x, p2.y);
    }

    Ok(())
}
