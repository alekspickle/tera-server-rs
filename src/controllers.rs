use std::io;
use std::time::Instant;
use serde_derive::Deserialize;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
pub struct Fibonacci {
    n: u32,
}

#[derive(Debug)]
pub struct Triplet {
    body: String,
    time: u128,
}

impl Triplet {
    pub fn body(&self) -> String {
        self.body.clone()
    }
    pub fn time(&self) -> u128 {
        self.time
    }
}

impl Triplet {
    fn new(body: &str, dur: u128) -> Triplet {
        Triplet {
            body: String::from(body).clone(),
            time: dur,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

pub fn draw_rectangle(width: &str, height: &str) -> String {
    let width: u32 = width.trim().parse().expect("Please type a number!");
    let height: u32 = height.trim().parse().expect("Please type a number!");
    let rect = Rectangle { width, height };
    let mut w = String::from("");
    for x in 0..=height {
        //left
        if x == 0 {
            w.push_str(" ")
        } else {
            w.push_str("|")
        }

        //middle
        for _y in 0..=width {
            if x == 0 || x == height {
                w.push_str("_")
            } else {
                w.push_str(" ")
            }
        }

        //right
        if x == 0 {
            w.push_str(" ")
        } else {
            w.push_str("|")
        }
    }
    w
}

pub fn fibonacci_number(n: i32) -> f64 {
    println!("Enter number of Fibonacci sequence, you want: ");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line.");

    let n: f64 = n
        .trim()
        .parse()
        .expect("Please type a number in f64 format.");

    let sq_five: f64 = f64::powf(5.0, 0.5);
    let fibonacci: f64 =
        ((1.0 + sq_five).powf(n) - (1.0 - sq_five)) / (f64::powf(2.0, n) * sq_five);
    println!(
        "Here is {}th number of the Fibonacci sequence: {}",
        n, fibonacci
    );

    fibonacci
}

pub fn pythagorian_triplets(n: &str) -> Triplet {
    let moment = Instant::now();
    let mut triplets = String::new();
    let n = n.trim().parse::<usize>().expect("Not parseable to usize");

    triplets = (0..)
        .map(|z| {
            (1..=z)
                .map(move |x| {
                    (x..=z)
                        .map(move |y| (x, y, z))
                        .filter(move |(x, y, z)| x * x + y * y == z * z)
                })
                // .filter_map(move |y|
                // if x*x + y*y == z*z {
                //     Some((x,y,z))
                // }else{
                //     None
                // }
                // ))
                .flatten()
        })
        .flatten()
        .take(n)
        .fold(String::new(), |acc, x| {
            println!("triplets {:?} next {:?}", acc, x);
            format!("{} ({}, {}, {});", acc, x.0, x.1, x.2)
        });

    Triplet::new(&triplets, moment.elapsed().as_micros())
}

pub fn celsius_to_fahrenheit(celsius: &str) -> String {
    let celsius = celsius
        .trim()
        .parse::<f64>();

    match celsius {
        Ok(val) => {
            let fahrenheit: f64 = val * 9.0 / 5.0 + 32.0;
            fahrenheit.to_string()
        }
        Err(why) => format!("Sorry, could not parse your number: {}", why)
    }
}

pub fn fahrenheit_to_celsius(f: &str) -> String {
    let mut fahrenheit = f
        .trim()
        .parse::<f64>();

    match fahrenheit {
        Ok(val) => {
            let celsius: f64 = val * 9.0 / 5.0 + 32.0;
            celsius.to_string()
        }
        Err(why) => format!("Sorry, could not parse your number: {}", why)
    }
}

pub fn get_christmas_lyrics() -> String {
    let mut output = String::from("");
    let cuplets: [String; 12] = [
        "first".to_string(),
        "second".into(),
        "third".into(),
        "fourth".into(),
        "fifth".into(),
        "sixth".into(),
        "seventh".into(),
        "eighth".into(),
        "ninth".into(),
        "tenth".into(),
        "eleventh".into(),
        "twelfth".into(),
    ];
    for x in 0..cuplets.len() {
        output = format!(
            "{}On the {} day of Christmas\nmy true love sent to me:\n",
            output, cuplets[x]
        );

        if x > 10 {
            output = format!("{}{}", output, "12 Drummers Drumming\n");
        }
        if x > 9 {
            output = format!("{}{}", output, "11 Pipers Piping\n");
        }
        if x > 8 {
            output = format!("{}{}", output, "10 Lords a Leaping\n");
        }
        if x > 7 {
            output = format!("{}{}", output, "9 Ladies Dancing\n");
        }
        if x > 6 {
            output = format!("{}{}", output, "8 Maids a Milking\n");
        }
        if x > 5 {
            output = format!("{}{}", output, "7 Swans a Swimming\n");
        }
        if x > 4 {
            output = format!("{}{}", output, "6 Geese a Laying\n");
        }
        if x > 3 {
            output = format!("{}{}", output, "5 Golden Rings\n");
        }
        if x > 2 {
            output = format!("{}{}", output, "4 Calling Birds\n");
        }
        if x > 1 {
            output = format!("{}{}", output, "3 French Hens\n");
        }
        if x > 0 {
            output = format!("{}{}", output, "2 Turtle Doves\n");
        }
        if x == 0 {
            output = format!("{}{}", output, "A Partridge in a Pear Tree!\n");
        } else {
            output = format!("{}{}", output, "and a Partridge in a Pear Tree!\n");
        }
    }

    output
}
