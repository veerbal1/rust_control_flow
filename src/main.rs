mod functions;

fn main() {
    let number = 22;
    let number_outcome = functions::sign_classifier(number);
    println!("Number is {}:", number_outcome);
}
