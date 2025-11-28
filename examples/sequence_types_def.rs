fn main() {
    let warmers: (&str, &str) = ("pink", "white");
    println!(
        "mai arm warmers are {} and {} striped",
        warmers.0, warmers.1
    );
    let isle: [f32; 3] = [5.00, 10.00, 49.99];
    println!("he picked de ${} collar for me :3", isle[2]);
    let leftover: &[f32] = &isle[0..2];
    println!("only {} more good boys can be rewarded!", leftover.len());
}
