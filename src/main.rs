fn add(n1: i8, n2: i8) -> i8 {
    n1 + n2
}

fn main() {
    let n = add(1, 2);
    println!("n: {}", n);
}

#[cfg(test)]
mod test {
    use super::add;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
