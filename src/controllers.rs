use serde_derive::Deserialize;
use std::time::Instant;

#[derive(Debug)]
struct _Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
pub struct AppData {
    pub count: u32,
}
#[derive(Debug, Deserialize)]
pub struct NForm {
    pub n: String,
}
#[derive(Debug, Deserialize)]
pub struct ConvertForm {
    pub temp: String,
}
#[derive(Debug, Deserialize)]
pub struct MultipartForm {
    pub image: String,
}

#[derive(Debug)]
pub struct Triplet {
    pub body: String,
    pub time: u128,
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
    fn new(body: String, dur: u128) -> Triplet {
        Triplet {
            body: body,
            time: dur,
        }
    }
}
impl AppData {
    pub fn _new(n: u32) -> AppData {
        AppData { count: n }
    }
    pub fn _increment(&mut self) {
       self.count += 1;
    }
}

impl _Rectangle {
    fn _area(&self) -> u32 {
        self.width * self.height
    }
    fn _can_hold(&self, rect: &_Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

pub fn _draw_rectangle(width: &str, height: &str) -> String {
    let width: u32 = width.trim().parse().expect("Please type a number!");
    let height: u32 = height.trim().parse().expect("Please type a number!");
    let _rect = _Rectangle { width, height };
    let mut w = String::from("");
    for x in 0..=height {
        //left
        if x == 0 {
            w.push_str(" ")
        } else {
            w.push_str("|")
        }

        //middle
        for _ in 0..=width {
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

pub fn fibonacci_number(n: String) -> String {
    let n = n.trim().parse::<f64>();

    match n {
        Ok(n) => {
            let sq_five: f64 = f64::powf(5.0, 0.5);
            let fibonacci: f64 =
                ((1.0 + sq_five).powf(n) - (1.0 - sq_five)) / (f64::powf(2.0, n) * sq_five);
            format!(
                "Here is {}th number of the Fibonacci sequence: {}",
                n, fibonacci
            )
        }
        Err(_) => "Please type a number in 0.0 format.".to_owned(),
    }
}

pub fn pythagorian_triplets(n: &str) -> Triplet {
    let moment = Instant::now();
    let n = n.trim().parse::<usize>().expect("Not parseable to usize");

    let triplets = (0..)
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
            // println!("triplets {:?} next {:?}", acc, x);
            format!("{} ({}, {}, {});", acc, x.0, x.1, x.2)
        });

    Triplet::new(triplets, moment.elapsed().as_micros())
}

pub fn celsius_to_fahrenheit(celsius: &str) -> String {
    let celsius = celsius.trim().parse::<f64>();

    match celsius {
        Ok(val) => {
            let fahrenheit: f64 = val * 9. / 5. + 32.;
            fahrenheit.to_string() + "°F"
        }
        Err(why) => format!("Sorry, could not parse your number: {}", why),
    }
}

pub fn fahrenheit_to_celsius(f: &str) -> String {
    let fahrenheit = f.trim().parse::<f64>();

    match fahrenheit {
        Ok(val) => {
            let celsius: f64 = (val - 32.) * 5. / 9.;
            celsius.to_string() + "°C"
        }
        Err(why) => format!("Sorry, could not parse your number: {}", why),
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

