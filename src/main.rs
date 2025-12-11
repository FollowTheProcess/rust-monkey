#![doc = include_str!("../README.md")]

fn main() {
    println!("Hello world");
}

#[cfg(test)]
mod tests {

    #[test]
    fn something() {
        assert_eq!(1 + 1, 2);
    }
}
