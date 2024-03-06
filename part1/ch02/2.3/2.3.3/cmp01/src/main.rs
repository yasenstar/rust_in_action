fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // compare when a and b in different type: will have "mismatched types" erro
    // if a < b {
    //     println!("10 is less than 100.");
    // }

    // compare when a and b in same type
    if a < (b as i32) {
        println!("10 is less than 100.");
    }

    // if (a as u16) < b {
    //     println!("10 is less than 100.");
    // }
}
