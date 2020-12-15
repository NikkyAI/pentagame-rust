pub fn check_password(password: &str) -> bool {
    if password.len() > 6_usize {
        // check by going over chars and checking if one number, one uppercase and on lowercase is satisfied
        let mut number = false;
        let mut uppercase = false;
        let mut lowercase = false;
        let mut ascii = true;
        for character in password.chars() {
            if !character.is_ascii() {
                ascii = false;
            } else if character.is_ascii_digit() {
                number = true;
            } else if character.is_ascii_lowercase() {
                lowercase = true;
            } else if character.is_ascii_uppercase() {
                uppercase = true;
            }
        }

        if !ascii || !lowercase || !uppercase || !number {
            println!(
                "u: {} l: {} n: {} a: {}",
                uppercase, lowercase, number, ascii
            );
            return true;
        } else {
            return false;
        }
    } else {
        println!("U: {}", password.len());
        return true;
    }
}
