// File: src/main.rs
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hasil: {}", tambah(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tambah() {
        assert_eq!(tambah(2, 2), 4);
    }
}
