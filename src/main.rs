fn main()
{
    let numbers = 30..51;
    for i in numbers {
        println!("The number is {}", i);
    }
    let animals = vec!["Rabbit", "Dog", "Cat"];
    for (index, animal) in animals.iter().enumerate() {
        println!("{} - The animal name is {}", index + 1, animal);
    }
}
