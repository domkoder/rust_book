

fn main() {
    // Integers
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A';

    let f: u8 = 255;

    println!("this are integers: {a}, {b}, {c}, {d}, {e}, {f}");

    // Floating Point numbers
    let x: f64 = 2.078; // f64
    let y: f32 = 3.056; // f32

    println!("this are the floating point numbers: {x}, {y}");

    // addition
    let sum: i32 = 5 + 10;
    println!("Sum: {sum}");

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    println!("Difference: {difference}");

    // multiplication
    let product: i32 = 4 * 30;
    println!("Product: {product}");

    // division
    let quotient: f64 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3;
    println!("Division -> quotient: {quotient} \ntruncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {remainder}");

    // The Boolean Type 
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Boolean -> True: {t} \nFalse: {f}");

    // The Character Type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Character: {c} {z} {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    
    println!("{x}, {y}, {z}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{five_hundred}, {six_point_four}, {one}");

    // The Array Type
    let arg: [i32; 5] = [1,2,3,4,5];
    let arg3: [i32; 5] = [3; 5];

    println!("{:?}", arg);
    println!("{:?}", arg3);

    // Functions
    another_function();
    print_labeled_measurment(48, 'h');

    // Statements and Expressions
    let y: i32 = {
        let x = 3;
        x+1
    };

    println!("The value of y is: {y}");

    // Functions with Return Values
    let fnc: i32 = five();
    println!("The value of fnc is: {fnc}");

    let fnc2 = plus_one(10);
    println!("The value of fnc2 is: {fnc2}");

    // CONTROL FLOW
    // if Expressions
    let number = 7;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number} ");

    // Repeating Code with loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop Labels to Disambiguate Between Multiple loops
    let mut count = 0;
    println!("my loop start here");
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10; 

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut while_number = 3;

    while while_number != 0 {
        println!("{while_number}");

        while_number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a collection with for
    let loop_a = [10, 20, 30, 40, 50];

    for element in loop_a {
        println!("the value is: {element}")
    }

    // Converting temperatures Between Fahrenheit and Celsius.
    let answer = temperature_converter(32.0, "F");
    println!("The temprature is {:.1}", answer);

    // Generate the nth Fibonacci number
    let nth = 10;
    let nth_fibonacci = fibonacci(nth);
    println!("The {nth} fibonaaci number is {nth_fibonacci}");

    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    twelve_days();


}

fn another_function(){
    println!("Another function.");
}

fn print_labeled_measurment(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32 ) -> i32  {
    x + 1
}

fn temperature_converter(temperature: f64, temperature_unit: &str) -> f64  {
    const C: &str = "C";
    const F: &str = "F";

    if temperature_unit == F {
        // If the temperature unit is Fahrenheit then we will convert to Celsius
        (temperature - 32.0) * 5.0/9.0
    } else if temperature_unit == C {
        // If the temperature unit is Celsius then we will convert to Celsius Fahrenheit
        (temperature * 1.8) + 32.0
    } else {
        0.0
    }
}

fn fibonacci(number: i32) -> i32 {
    let mut previous_result = 0;
    let mut result = 1;
    let mut current_result;
    let mut index = 2;

    if number == 0 {
        return previous_result
    }else {
        while index <= number {
            current_result = previous_result + result;

            previous_result = result;

            result = current_result;

            index += 1;
        }
    }
    return result;

}

fn  twelve_days() {
    
    let gifts : [&str; 12] = ["And a partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,","Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    let days: [&str; 12] = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for (day_index, day) in days.iter().enumerate() {
        let mut current_day = day_index+1;

        println!("\n{}. On the {day} day of Christmas \nMy true love gave to me", current_day);


        while current_day  != 0 {
            if day_index == 0 {
                println!("A partridge in a pear tree");
                break;
            }
            println!("{}", gifts[current_day-1]);
            current_day -= 1;
        }
    }

}