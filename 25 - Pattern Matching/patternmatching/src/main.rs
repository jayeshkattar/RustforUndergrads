fn main() {
    let variable_name = String::from("Jayesh");

    println!("Char at posotion 3 is {}",
    match variable_name.chars().nth(3) {
        Some(v) => v.to_string(),
        None => "No character at this positon".to_string(),
    });

}
