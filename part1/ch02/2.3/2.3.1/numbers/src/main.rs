fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32; // also OK for 22_i32

    let addition = twenty + twenty_one + twenty_two;

    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_trillion: i128 = 1_000_000_000_000;

    println!("{} and {}", one_trillion, one_trillion.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32
    ];

    println!("{}, {}", forty_twos[2], forty_twos[1])
}
