fn main() {
    let len: u8 = 10;
    println!("Silnia z: {}", len);
    println!();
    println!("Wynik: {}", pow(len));
    println!();
}

pub fn pow(len: u8) -> u64 {
    assert!(len <= 20, "Max silnia to 20");
    let mut result: u64 = 1; 
    for i in 1..len+1{
        result *= i as u64;
        println!("{}. {}",i , result); 
    }
    println!();
    result
}
