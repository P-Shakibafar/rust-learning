fn main()
{
    let name = String::from("Hasta");
    println!("Character at index 1:{}",
             match name.chars().nth(1) {
                 Some(c) => c.to_string(),
                 None => "No character at index 8!".to_string()
             }
    );
    println!("Occupation is {}",
             match get_occupation(&name) {
                 Some(o) => o,
                 None => "No occupation found!"
             }
    );
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Hasta" => Some("Web Dev"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}
