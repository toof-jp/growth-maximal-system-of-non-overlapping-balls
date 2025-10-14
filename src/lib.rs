pub struct Points(Vec<Point>);

pub struct Point(Vec<f64>);

pub struct Radius(Vec<f64>);

pub fn f(points: Points) -> Radius {
    let mut radius = Radius(vec![0.0; points.0.len()]);
    
    let len = points.0.len();
    
    for _ in 0..len {
        let mut min_distance = (f64::MAX, None);
        
        for i in 0..len {
            for j in 0..len {
                if i != j && !(radius.0[i] == 0.0 && radius.0[j] == 0.0) {
                    let distance = distance(&points.0[i], &points.0[j]);
                    if distance < min_distance.0 {
                        min_distance = (distance, Some((i, j)));
                    }
                }
            }
        }
        
        if let Some((i, j)) = min_distance.1 {
            if radius.0[i] == 0.0  {
                radius.0[i] = min_distance.0;
            } else if radius.0[j] == 0.0 {
                radius.0[j] = min_distance.0;
            } else {
                radius.0[i] = min_distance.0 / 2.0;
                radius.0[j] = min_distance.0 / 2.0;
            }
        }
    }
    
    radius
}

fn distance(point1: &Point, point2: &Point) -> f64 {
    let mut distance = 0.0;
    for i in 0..point1.0.len() {
        distance += (point1.0[i] - point2.0[i]).powi(2);
    }
    distance.sqrt()
}
