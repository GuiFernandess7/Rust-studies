#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn show_direction(dir: Direction) -> (){
    println!("Direction: {:?}", dir);
}

impl Direction {
    fn double(&self){
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        };
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    println!("Rectangle: {:?}", rect);

    show_direction(Direction::North);
    show_direction(Direction::East);

    let doubled_dir = Direction::North.double();
    println!("Doubled direction: {:?}", doubled_dir);
}
