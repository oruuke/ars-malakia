fn main() {
    // char holds but one character
    let confusion: char = '?';
    println!("guh{}", confusion);
    // str is dynamically sized type so requires pointer
    let greeting: &str = "hai world!";
    println!("ferris exclaims wit excitement, \"{}\"", greeting);
    // unsigned ints include u8 up to u128, same wit signed
    let count: u8 = 69;
    println!("woah, {} blahaj!!", count);
    // floats only got f32 and f64
    let measurement: f32 = 1.25;
    println!("awwww only {}L left of milk...", measurement);
    // booleans use lowercase keyword
    let boolean: bool = true;
    println!("programming socks boost code quality: {}", boolean);
}
