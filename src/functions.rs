pub fn sign_classifier(input: i32) -> String {
    if input < 0 {
        return String::from("Negative");
    } else if input > 0 {
        return String::from("Positive");
    } else {
        return String::from("Zero");
    }
}

pub fn fizz_buzz() {
    for i in 1..=100 {
        if (i % 3 == 0) && (i % 5 == 0) {
            println!("FizzBuzz {}", i);
        }
        if i % 3 == 0 {
            println!("Fizz {}", i);
        }
        if i % 5 == 0 {
            println!("Buzz {}", i);
        }
    }
}

pub fn grade_to_letter() {
    println!("--------------");
    for i in 1..=100 {
        match i {
            90..=100 => {
                println!("A")
            }
            80..90 => {
                println!("B")
            }
            60..80 => {
                println!("C")
            }
            50..60 => {
                println!("D")
            }
            34..50 => {
                println!("E")
            }
            _ => {
                println!("F")
            }
        }
    }
}

pub fn collatz(input: u128) {
    println!("----collatz------");
    let mut i = input;
    let mut step = 0;

    while i != 1 {
        step += 1;
        if i % 2 == 0 {
            println!("{} {}", i, i / 2);
            i = i / 2;
        } else {
            println!("{} {}", i, 3 * i + 1);
            i = 3 * i + 1;
        }
    }
    println!("Step {}", step);
}
