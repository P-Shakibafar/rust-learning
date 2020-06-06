#![allow(dead_code)]

enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Day {
    fn is_weekday(&self) -> bool {
        return match self {
            &Day::Sat | &Day::Sun => false,
            _ => true
        };
    }
}

fn main()
{
    let d = Day::Tue;
    println!("Is d weekday? {}", d.is_weekday());
}
