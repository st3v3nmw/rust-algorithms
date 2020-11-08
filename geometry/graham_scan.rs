// TODO

fn convex_hull_graham_scan<T: Ord + Copy + std::fmt::Display + std::fmt::Debug + std::ops::Mul<Output = T> + std::ops::Add<Output = T>>(points: &mut [(T, T)]) -> Vec<(T, T)> {
    // find point p with lowest x in the set of points with lowest y
    let p: (T, T) = *points.iter().min_by(|p1, p2| p1.1.cmp(&p2.1).cmp(&p2.0.cmp(&p1.0))).unwrap();

    println!("{} {}", p.0, p.1);

    println!("{:?}", points);

    let mut convex_hull: Vec<(T, T)> = vec![];

    return convex_hull;
}

fn main() {
    let mut points: Vec<(i32, i32)> = vec![(5, -2), (4, -4), (3, 3), (1, 3), (0, 1), (7, 4), (9, -1), (-2, 1), (-2, -2), (8, -4), (8, 1), (1, -4), (7, 1), (5, 2), (4, 1), (1, -2)];
    convex_hull_graham_scan(&mut points);
}