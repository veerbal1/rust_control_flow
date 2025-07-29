mod functions;

fn main() {
    let number = 22;
    let number_outcome = functions::sign_classifier(number);
    println!("Number is {}:", number_outcome);

    functions::fizz_buzz();

    functions::grade_to_letter();

    let number: u128 = 99999999999999999999999999999999;
    functions::collatz(number);
}
