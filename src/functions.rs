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
