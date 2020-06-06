mod hasta {
    fn chicken() {
        println!("Chicken!");
    }

    pub fn print_message() {
        chicken();
        println!("How's it going!");
    }

    pub mod water {
        pub fn print_message() {
            println!("I'm water!");
        }
    }
}

fn main()
{
    hasta::print_message();
    hasta::water::print_message();
}
