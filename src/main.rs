use std::collections::HashMap;

fn main()
{
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web Dev", 94);
    marks.insert("UX Design", 54);

    println!("How many subjects have you studied? {}\n", marks.len());

    match marks.get("Web Dev") {
        Some(mark) => println!("You got {} for Web Dev\n", mark),
        None => println!("You didn't study Web Dev\n")
    }

    marks.remove("UX Design");


    for (subject, mark) in marks.iter() {
        println!("{} is {}", subject, mark);
    }
    println!("\n");
    for (subject, mark) in &marks {
        println!("For {} you go {}%!", subject, mark);
    }
    println!("\n");

    println!("Did you study C++? {}", marks.contains_key("C++ Programing"));
}

