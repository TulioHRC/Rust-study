struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other_rectangle: Rectangle) -> bool {
        return self.width >= other_rectangle.width && self.height >= other_rectangle.height; 
    }
}

fn main() {
    let rect = Rectangle {
        width: 100,
        height: 200
    };

    println!("We have an origional rectangle with {0} width and {1} height.\n", rect.width, rect.height);

    println!("The rectangle area is {0}. \nThis was made by {1}, and the area is {0}.", rect.calculate_area(), "Tulio");

    let mut new_rect_width = String::new();
    let mut new_rect_height = String::new();

    println!("Type a width: ");
    std::io::stdin()
        .read_line(&mut new_rect_width)
        .expect("Failed to read line!");
    let new_rect_width : u32 = new_rect_width.trim().parse().unwrap_or(0);

    println!("Type a height: ");
    std::io::stdin()
        .read_line(&mut new_rect_height)
        .expect("Failed to read line!");
    let new_rect_height : u32 = new_rect_height.trim().parse().unwrap_or(0);

    let new_rect = Rectangle {
        width: new_rect_width,
        height: new_rect_height
    };

    println!("Can the original rectangle fit yours? {}", rect.can_hold(new_rect));
}
