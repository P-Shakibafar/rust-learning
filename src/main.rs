// Lesson 10
/*
enum Direction {
Up,
Down,
Left,
Right,
}
*/
// Lesson 11
/*
const MAXIMUM_NUMBER: u8 = 20;
*/
// Lesson 17
/*
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
 */

fn main()
{
    // Lesson 2 , 3
    /*
    a sample hello worlds project with rust.
    just print Hello Hasta!
    println!("Hello Hasta!");
    */

    // Lesson 4
    /*
    let mut x = 45;
    let y = false;

    println!("the value of x is {} and y is {}", x, y);

    x = 55;
    println!("the value of x is {} and y is {}", x, y);
    */

    // Lesson 5
    /*
    let integer = 45; // i8 ,i16 ,i32 ,i64 ,i128 ,isize =>i:integer
    let unsigned_integer: u64 = 45; // u8 ,u16 ,u32 ,u64 ,u128 ,usize =>u:unsigned integer
    let float = 6.4; // f32 , f64 => f:float
    let boolean = false; // bool =>bool:boolean
    let string = "hasta";
    let char = 'c';
    */

    // Lesson 6
    /*
    let n = 22;
    if n < 30 {
    println!("The number is less then 30!");
    } else if n == 30 {
    println!("Rust!");
    } else {
    println!("It was great then 30!");
    }
    */

    // Lesson 7
    /*
        let mut n = 0;
        loop {
            n += 1;

            if n == 7 {
                continue;
            }

            if n > 10 {
                break;
            }

            println!("The value of n is {}", n);
        }
         */

    // Lesson 8
    /*
    let mut n = 1;

    while n <= 50 {
        // if n is a multiple of 5
        if n % 5 == 0 {
            println!("n is {}", n);
        }
        n += 1;
    }
     */

    // Lesson 9
    /*
    let numbers = 30..51;
    for i in numbers {
        println!("The number is {}", i);
    }
    let animals = vec!["Rabbit", "Dog", "Cat"];
    for (index, animal) in animals.iter().enumerate() {
        println!("{} - The animal name is {}", index + 1, animal);
    }
     */

    // Lesson 10
    /*
    let player_direction: Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are going all the may down..!"),
        Direction::Left => println!("Left is right!"),
        Direction::Right => println!("Moving towards the right!"),
    }
     */

    // Lesson 11
    /*
    for n in 1..MAXIMUM_NUMBER {
        println!("{}", n);
    }
     */

    // Lesson 12
    /*
    let tup1 = (20, "34", false, 6.5, (1, 3, 6));
    println!("{}", (tup1.4).1);

    let tup2 = (45, 6.8, "Rust");
    let (a, b, c) = tup2;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
     */

    // Lesson 13
    /*
    print_numbers_to(12);
    if is_even(40) {
        println!("It is even!");
    } else { println!("It is odd"); }
     */

    // Lesson 14
    /*
    let x = 10;
    let y = {
        let y = 5;
        y
    };
    println!("x: {} y: {}", x, y);
     */

    // Lesson 15
    /*
    let mut x = 10;
    {
        x = 15;
    }

    let x = "X is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
     */

    // Lesson 16
    /*
    let mut x = 10;
    // let xr = &x;
    {
        let dom = &mut x;
        *dom += 1;
    }
    println!("x is {}", x);
     */

    // Lesson 17
    /*
    let mut  bg = Color { red: 255, green: 70, blue: 15 };
    bg.blue = 45;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
     */
}

// Lesson 13
/*
fn print_numbers_to(number: u32) {
    for n in 1..number {
        if is_even(n) {
            println!("{} is even!", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
 */
