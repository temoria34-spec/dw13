use dw13::wagon_total;

fn main() {
    let miles = [[12, 15, 9], [10, 11, 14], [8, 13, 16]];
    println!("Wagon 0 total: {}", wagon_total(&miles, 0));
}
