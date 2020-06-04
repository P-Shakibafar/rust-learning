fn main()
{
    // let my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4];

    my_vector.push(34);
    my_vector.remove(1);

    for number in my_vector.iter() {
        println!("{}", number);
    }
}

