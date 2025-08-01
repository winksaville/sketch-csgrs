//! sketch-csgrs
use clap::Parser;
use csgrs::{float_types::Real, sketch::Sketch};

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    width: Real,
    height: Real,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Enable printing Mesh"
    )]
    print_mesh: bool,
}

fn main() {
    println!("sketch-csgrs:+");

    // Parse command line arguments
    let args = Args::parse();

    // Create a square
    let width: Real = args.width;
    let height = args.height;
    let square_sketch: Sketch<()> = Sketch::square(width, None);
    let square = square_sketch.extrude(height);
    if args.print_mesh {
        println!("square Mesh: {square:?}");
    }

    // Save the mesh as an STL file
    let name = "extrude_square";
    let square_stl = square.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", square_stl).unwrap();

    println!("sketch-csgrs:-");
}
