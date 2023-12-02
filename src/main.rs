use rand::Rng;

fn main() {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+";
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let mut count = 0;
    let mut password_length_string = String::new();

    println!("How long do you want your password to be?");
    std::io::stdin().read_line(&mut password_length_string).expect("Failed to read line");
    let password_length = password_length_string.trim().parse().expect("Please type a number!");

    while count < password_length {
        let character: char = characters.chars().nth(rng.gen_range(0..characters.len())).unwrap();
        password.push(character);
        count += 1;
    }

    println!("Your password is: {}", password);
    
}
