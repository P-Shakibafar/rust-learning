fn main()
{
    let mut x = 10;
    {
        x = 15;
    }

    let x = "X is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
}
