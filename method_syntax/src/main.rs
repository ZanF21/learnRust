#[derive(Debug)]
struct Quad {
    length : u32,
    breadth : u32
}

impl Quad {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
    fn perimeter(&self) -> u32 {
        2 * (self.length + self.breadth)
    }
    fn square(length : u32) -> Quad {
        Quad { 
            length, 
            breadth : length 
        }
    }
}



fn main() {
    let sq=Quad::square(5);
    let rec=Quad {
        length : 5,
        breadth : 10
    };
    println!("{:?}",sq);
    println!("Area of square is {}",sq.area());
    println!("Perimeter of square is {}",sq.perimeter());
    println!("sq can hold rec? {}",sq.can_fit(&rec));
    println!("{:?}",rec);
    println!("Area of rectangle is {}",rec.area());
    println!("Perimeter of rectangle is {}",rec.perimeter());
    println!("rec can hold sq? {}",rec.can_fit(&sq));
}


impl Quad {
    fn can_fit(&self, other: &Quad) -> bool {
        self.length.min(self.breadth) >= other.length.min(other.breadth) && self.length.max( self.breadth) >= other.length.max(other.breadth)
    }
}