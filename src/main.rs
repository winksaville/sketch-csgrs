//! sketch-csgrs
use csgrs::{float_types::Real, sketch::Sketch};

fn main() {
    println!("sketch-csgrs:+");

    // Create a square
    let width = Real::NAN;
    let height = 10.0;
    let square_sketch: Sketch<()> = Sketch::square(width, None);
    let square = square_sketch.extrude(height);
    println!("Generated mesh: {square:?}");

    // Save the mesh as an STL file
    let name = "extrude_square";
    let square_stl = square.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", square_stl).unwrap();

    println!("sketch-csgrs:-");
}
