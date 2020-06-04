fn main()
{
    let mut my_string = String::from("How is it going? My name is Hasta.");

    // Length
    println!("Length: {}", my_string.len());
    // Is Empty>
    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!("Does the string contain 'Hasta'? {}", my_string.contains("Hasta"));
    my_string.push_str(" Welcome to your tutorial on Strings!");
    println!("{}", my_string);
}
