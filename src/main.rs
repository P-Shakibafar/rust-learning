struct Rectangle {
    with: u32,
    height: u32,
}

impl Rectangle {
    fn print_desc(&self) {
        println!("Rectangle: {} x {}", self.with, self.height);
    }

    fn is_square(&self) -> bool {
        self.with == self.height
    }
}

fn main()
{
    let my_rect = Rectangle { with: 5, height: 5 };

    my_rect.print_desc();
    println!("Rectangle is a square: {}", my_rect.is_square());
}
