fn main()
{
    let tup1 = (20, "34", false, 6.5, (1, 3, 6));
    println!("{}", (tup1.4).1);

    let tup2 = (45, 6.8, "Rust");
    let (a, b, c) = tup2;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}
