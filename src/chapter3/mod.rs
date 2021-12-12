use std::io;
use std::collections::HashMap;

fn f_to_c_convert(f: f32) -> f32 {
    (f - 32.0) / 9.0 * 5.0
}

fn fib(n: i32, d: &mut HashMap<i32, i32>) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        if d.contains_key(&n) {
            return d[&n];
        } else {
            let m = fib(n - 1, d) + fib(n - 2, d);
            d.insert(n, m);
            return m;
        }
    }
}

fn twelve_days_of_christmas() -> () {
    let days = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Nineth",
        "Tenth", "Eleventh", "Twefth",
    ];
    let bring_to_me = [
        "A song and a Christmas tree.",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];

    for i in 0..days.len() {
        println!("{}", f!("On the {day} day of Christmas", day = days[i]));
        println_f!("My good friends brought to me");
        for j in (0..=i).rev() {
            println!("{}", f!("{bring}", bring = bring_to_me[j]));
        }
        println!()
    }
}

pub fn chapter3() {
    println!("Input your degrees in Fahrenheit: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = input.trim().parse().expect("Please type a number");
    let deg_c = f_to_c_convert(input);
    println_f!("Degrees in Celsius: {deg_c}");

    println!("Input the nth Fibonacci you want to compute: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please type a number");
    let mut dict_fib: HashMap<i32, i32> = HashMap::new();
    let fib_n = fib(input, &mut dict_fib);
    println_f!("{input}th Fib number is: {fib_n}");

    println!("A Song And A Christmas Tree:\n");
    twelve_days_of_christmas();
}
