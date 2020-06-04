fn main()
{
    let x = 10;
    let y = {
        let y = 5;
        y
    };
    println!("x: {} y: {}", x, y);
}
