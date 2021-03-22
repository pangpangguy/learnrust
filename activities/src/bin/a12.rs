// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Brown,
}

impl Color{
    fn print(&self){
        match self{
            Color::Red => println!("Red"),
            Color::Brown => println!("Brown"),
        }
    }
}

struct Dimensions{
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions{
    fn print(&self){
        println!("{}x{}x{}", self.height, self.width, self.depth);
    }
}
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(d: Dimensions, w: f64, c: Color) -> Self {
        Self {
            dimensions: d,
            weight: w,
            color: c,
        }
    }

    fn print(&self){
        self.dimensions.print();
        self.color.print();
        println!("Weight is {:?}", self.weight);
    }
}
fn main() {
    let new_dimension = Dimensions{
        depth:5.0, width:5.0, height:4.5,
    };
    
    let newbox = ShippingBox{
        dimensions: new_dimension,
        weight: 5.0,
        color: Color::Red,
    };

    newbox.print();
    //Create new box using function
    let newbox2 = ShippingBox::new(newbox.dimensions, 4.0, Color::Red);
    newbox2.print();
}
