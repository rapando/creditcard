use regex::Regex;

/// Defines the accepted card types. None means the card type is invalid
#[derive(Debug, PartialEq)]
pub enum CardType {
    Amex,
    MasterCard,
    Visa,
    None,
}

/// used as base when changing strings to digits.
const RADIX: u32 = 10;

/// Tries to guess the correct card type from regex
///
/// # Examples
///
/// ```
/// use regex::Regex;
///
/// #[derive(Debug, PartialEq)]
/// enum CardType {
///     Amex,
///     MasterCard,
///     Visa,
///     None,
/// }
///
/// let card_number:String = String::from("378282246310005");
/// let possible_card_type:CardType = get_possible_card_type(&card_number);
/// ```
pub fn get_possible_card_type(card_number: &String) -> CardType {
    let amex_regex = Regex::new(r"^(34|37)[0-9]{13}$").unwrap();
    let mastercard_regex = Regex::new(r"^5(1,2,3,4,5)[0-9]{14}$").unwrap();
    let visa_regex = Regex::new(r"^(4)[0-9]{13,15}$").unwrap();

    if amex_regex.is_match(card_number) {
        return CardType::Amex;
    }
    if mastercard_regex.is_match(card_number) {
        return CardType::MasterCard;
    }
    if visa_regex.is_match(card_number) {
        return CardType::Visa;
    }
    return CardType::None;
}

/// Used to get a patterned sum of the card digits.
/// First, get the double of every second digit from right to left
/// then add up the individual digits.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq)]
/// enum CardType {
///     Amex,
///     MasterCard,
///     Visa,
///     None,
/// }
///
/// let card_number:String = String::from("378282246310005");
/// let patterned_sum:u32 = get_patterned_sum(&card_number);
/// ```
pub fn get_patterned_sum(card_number: &str) -> u32 {
    let mut index: i32 = 0;
    let mut overall_sum: u32 = 0;

    let split_string = card_number.chars().rev();

    for c in split_string {
        if index % 2 == 1 {
            if let Some(char_value) = c.to_digit(RADIX) {
                let sum = char_value * 2;
                let sum_string = sum.to_string();
                overall_sum += add_up_digits_in_string(&sum_string);
            }
        }
        index += 1
    }
    overall_sum
}

/// Gets the sum of every 1st, 3rd etc digit from the right to left.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq)]
/// enum CardType {
///     Amex,
///     MasterCard,
///     Visa,
///     None,
/// }
///
/// let card_number:String = String::from("378282246310005");
/// let unused_digit_sum:u32 = get_unused_digit_sum(&card_number);
/// ```
pub fn get_unused_digit_sum(card_number: &str) -> u32 {
    let mut index: i32 = 0;
    let mut sum: u32 = 0;
    let split_string = card_number.chars().rev();

    for c in split_string {
        if index % 2 == 0 {
            if let Some(char_value) = c.to_digit(RADIX) {
                sum += char_value
            }
        }
        index += 1
    }
    sum
}

/// Adds up the individual digits in a string.
///
/// # Examples
///
/// ```
/// let s:String = String::new("1234");
/// let sum:u32 = add_up_digits_in_string(&s);
/// println!("The sum of digits is {} is {}", s, sum); // 1234, 10
/// ```
fn add_up_digits_in_string(s: &String) -> u32 {
    let mut sum: u32 = 0;
    for ch in s.chars() {
        if let Some(digit) = ch.to_digit(RADIX) {
            sum += digit;
        }
    }
    sum
}
