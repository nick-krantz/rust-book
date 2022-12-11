#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // no struct or types
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    // using tuples
    {
        let rect1 = (30, 50);

        println!(
            "the area of the rectangle is {} square pixels.",
            area_tuple(rect1)
        )
    }

    // use structs
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area_struct(&rect1)
        );

        println!("rect1 is {:?}", rect1); // prints everything on one line
        println!("rect1 is {:#?}", rect1); // pretty-print
    }

    // methods
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
