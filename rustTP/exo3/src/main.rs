#[derive(Debug)]
enum Couleur{
    Rouge,
    Vert,
    Bleu,
}

#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
    couleur: Option<Couleur>
}

fn main() {
    //Question 1
    let pt = Point{
        x: 0.0,
        y: 0.0,
        couleur: Some(Couleur::Rouge),
    };
    println!("Origin point : {:?}", pt);
    
    let pt2 = new(1.0, 2.0);
    println!("Create a new point 1,2 : {:?}", pt2);
    println!("Distance between (0,0) and (1,2) : {}", distance(&pt, &pt2));
    
    //Using implentation methode
    let pt3 = Point::new(1.0,2.0);
    println!("With other method");
    println!("Create new point : {:?}", pt3);
    println!("Distance from origin: {}", pt.distance(&pt3));
}

//Question 2
fn new(x:f64, y:f64) -> Point{
    Point{
        x,
        y,
        couleur: None,
    }
}

//Question 3
fn distance(pt1: &Point, pt2: &Point) -> f64{
    let mut distance = (pt1.x - pt2.x).powi(2) + (pt1.y - pt2.y).powi(2);
    distance = distance.sqrt();
    distance
}

//Question 2 and 3 alternative, done using implementation
//Implementation block, all struct Point methods go in here
impl Point{
    fn new(x:f64, y:f64) -> Self{
        Point{
            x,
            y,
            couleur: None,
        }
    }

    fn distance(self: &Self, other: &Self)-> f64{
        ((self.x-other.x).powi(2) + (self.y-other.y).powi(2)).sqrt()
    }
}
