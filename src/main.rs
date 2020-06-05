fn main()
{
    let number = 8;
    match number {
        1 => println!("It is one!"),
        2..=9 => println!("It is greater then one!"),
        10 | 11 => println!("It is either 10 or 11"),
        _ => println!("It doesn't match!")
    }
}
