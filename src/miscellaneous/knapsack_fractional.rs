use std::cmp::Ordering::Equal;

pub fn fractional_knapsack(weights: &Vec<f64>, values: &Vec<f64>, mut capacity: f64) -> f64 {
    let mut ratios: Vec<_> = weights.iter().zip(values).collect();
    ratios.sort_unstable_by(|(x1, y1), (x2, y2)| ((**y2 / **x2).partial_cmp(&(**y1 / **x1))).unwrap_or(Equal));

    let mut value: f64 = 0.0;
    for (x, y) in &ratios {
        if **x < capacity {
            value += **y;
            capacity -= **x;
        } else {
            value += capacity / **x * **y;
            break;
        }
    }
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fractional_knapsack_test() {
        assert_eq!(fractional_knapsack(&(vec![30.0, 30.0, 40.0, 1.0]), &(vec![5.0, 80.0, 50.0, 18.0]), 50.0), 121.75);
    }
}