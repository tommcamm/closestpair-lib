#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// Naive method
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


// v.2 passing strip to rec. to avoid reallocations
fn closest_pair_recursive(px: &[Point], py: &[Point], strip: &mut Vec<Point>) -> (Point, Point, f64) {
    let n = px.len();
    
    // Increased to 7
    if n <= 7 {
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

    let (l1, r1, dl) = closest_pair_recursive(lx, &ly, strip);
    let (l2, r2, dr) = closest_pair_recursive(rx, &ry, strip);
    let mut d = dl.min(dr);
    let mut closest_pair = if dl < dr { (l1, r1) } else { (l2, r2) };

    strip.clear();
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

pub fn closest_pair(points: &[Point]) -> (Point, Point, f64) {
    let mut px: Vec<Point> = points.to_vec();
    let mut py: Vec<Point> = points.to_vec();

    px.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    py.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut strip = Vec::with_capacity(points.len());
    closest_pair_recursive(&px, &py, &mut strip)
}
