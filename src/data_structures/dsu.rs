#[derive(Debug)]
pub struct DSU {
    v: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let mut v: Vec<usize> = vec![];
        for i in 0..n {
            v.push(i);
        }
        return DSU { v };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsu_test() {
        let mut dsu: DSU = DSU::new(10);
    }
}
