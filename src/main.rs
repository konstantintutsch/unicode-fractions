use std::{io, panic};

mod unicode;
use unicode::script::Numbers;

fn main() {
    let mut numerator = String::new();
    let mut denominator = String::new();

    println!("Numerator:");
    let numerator_result = io::stdin().read_line(&mut numerator);
    let numerator_n = match numerator_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read numerator from stdin: {error:?}")
    };

    println!("Denominator:");
    let denominator_result = io::stdin().read_line(&mut denominator);
    let denominator_n = match denominator_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read denominator from stdin: {error:?}")
    };

    // Remove trailing \n from input
    numerator.truncate(numerator_n - 1);
    denominator.truncate(denominator_n - 1);

    // Convert to super- or subscript
    numerator = numerator.to_superscript();
    denominator = denominator.to_subscript();

    let fraction = format!("{numerator}⁄{denominator}");
    println!("{fraction}")
}
