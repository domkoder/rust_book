

fn main() {
let config_max = Some(3u8);

// the tow block of code will do the same thing
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _=>(),
}

if let Some(max) = config_max{
    println!("The maximum is configured to be {}", max);
}

}