fn main() {
    let three = 0b11;
    let thirty = 30;
    let three_hundred = 300;

    println!("bsae 10: {} {} {}", three, thirty, three_hundred);

    println!("bsae 2: {:b} {:b} {:b} {:b}", three, thirty, three_hundred, three * thirty);

    println!("bsae 8: {:o} {:o} {:o}", three, thirty, three_hundred);

    println!("bsae 16: {:x} {:x} {:x}", three, thirty, three_hundred)
}
