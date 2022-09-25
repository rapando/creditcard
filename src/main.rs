/*
* Rapando
* Validating Credit Numbers using Luhn's Algorithm

https://docs.cs50.net/2018/x/psets/1/credit/credit.html#background

* Points to note:

? 1. Length of card number
- American Express  : 15
- MasterCard        : 16
- Visa              : 13, 14

? 2. First Digits
- American Express  : 34,37
- MasterCard        : 51,52,53,54,55
- Visa              : 4

* The process (Luhn's algorithm)
1. From right to left, multiply every second digit with 2 and then add up all the products
2. Add the sum (in 1) to the sum of all numbers that weren't multiplied by 2
3. if the total%10==0, the number is valid

*/

use std::io;
mod card;

fn main() {
    let mut cn: String = String::new();

    println!("Enter the card Number to check");

    let card_number = match io::stdin().read_line(&mut cn) {
        Ok(_) => {
            let trimmed_str = cn.trim();
            String::from(trimmed_str)
        }
        Err(e) => {
            println!("unable to read card number because {:?}", e);
            return;
        }
    };

    let card_type = card::get_possible_card_type(&card_number);

    if card_type == card::CardType::None {
        println!("Card number is invalid!");
        return;
    }

    let patterned_sum = card::get_patterned_sum(&card_number);
    let unused_digit_sum = card::get_unused_digit_sum(&card_number);

    let overall_sum = patterned_sum + unused_digit_sum;
    if overall_sum % 10 == 0 {
        println!("\nThe card is a valid {:?}!", card_type);
    } else {
        println!("\nThe card number {} is invalid!", card_number);
    }
}
