fn main() {
    println!("Welcome");

    println!("Number of times 'fizz buzz' occurred: {}", fizz_buzz());

}

fn fizz_buzz() -> u32 {
    let mut fizz_buzz_count = 0;

    for count in 1..=301 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
    }

    fizz_buzz_count
}