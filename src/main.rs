fn main()
{
    // let numbers = [1, 2, 3, 4];
    let numbers = [2; 20];

    // for number in numbers.iter() {
    //     println!("{}", number);
    // }
    for number in 0..numbers.len() {
        println!("{}", numbers[number]);
    }
}

