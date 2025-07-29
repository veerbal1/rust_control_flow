pub fn sign_classifier(input: i32) -> String {
    if input < 0 {
        return String::from("Negative");
    } else if input > 0 {
        return String::from("Positive");
    } else {
        return String::from("Zero");
    }
}
