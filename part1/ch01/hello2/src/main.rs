fn greeting_world() {
    println!("Hello, world!");

    let english = "Hello World in English!";

    let chinese = "你好，世界";

    let regions = [english, chinese];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greeting_world();
}
