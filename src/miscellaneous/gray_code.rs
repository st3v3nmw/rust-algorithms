pub fn gen(codes: &mut Vec<String>) -> Vec<String> {
    let mut reversed: Vec<String> = codes.to_vec();
    reversed.reverse();
    for i in 0..codes.len() {
        codes[i].insert(0, '0');
        reversed[i].insert(0, '1');
    }
    codes.extend(reversed);
    codes.to_vec()
}

pub fn gray_code(n: usize) -> Vec<String> {
    let mut codes: Vec<String> = vec![String::from("0"), String::from("1")];
    for _ in 0..n-1 {
        codes = gen(&mut codes);
    }
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_test() {
        let codes: Vec<String> = gray_code(3);

        for code in codes {
            println!("{}", code);
        }
    }
}