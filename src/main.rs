//! sketch-csgrs
use clap::Parser;
use csgrs::{float_types::Real, sketch::Sketch};
use serde::Serialize;

#[derive( clap::ValueEnum, Clone, Default, Debug, Serialize,)]
#[serde(rename_all = "kebab-case")]
enum SketchCommand {
    #[default]
    Square,
}

#[derive(Parser, Debug)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    cmd: SketchCommand,

    #[arg(
        short,
        long,
        default_value = "1",
        help = "Length of object (x direction)"
    )]
    length: Real,

    #[arg(
        short,
        long,
        default_value = "1",
        help = "Width of object (y direction)"
    )]
    width: Real,

    #[arg(
        short='t',
        long,
        default_value = "10",
        help = "Height of object (z direction)"
    )]
    height: Real,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Enable printing Mesh"
    )]
    print_mesh: bool,
}

fn square_rod(args: &Args) {
    println!("square_rod:+");

    // Create a square
    let width: Real = args.width;
    let height = args.height;
    let square_sketch: Sketch<()> = Sketch::square(width, None);
    let square_rod = square_sketch.extrude(height);
    if args.print_mesh {
        println!("square_rod Mesh: {square_rod:?}");
    }

    // Save the mesh as an STL file
    let name = "square_rod";
    let square_stl = square_rod.to_stl_ascii(name);
    std::fs::write(name.to_owned() + ".stl", square_stl).unwrap();

    println!("square_rod:-");
}

fn main() {
    println!("main:+");

    // Parse command line arguments
    let args = Args::parse();

    match args.cmd {
        SketchCommand::Square => {
            square_rod(&args);
        }
    }   

    println!("main:-");
}
