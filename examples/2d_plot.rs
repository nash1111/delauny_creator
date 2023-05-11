use rand::distributions::Uniform;
use std::collections::HashSet;
use std::env;

use plotters::prelude::*;
use rand::Rng;

fn triangle_to_vertices(triangle: &delaunay_creator::Triangle) -> Vec<(f64, f64)> {
    vec![
        (triangle.a.x, triangle.a.y),
        (triangle.b.x, triangle.b.y),
        (triangle.c.x, triangle.c.y),
    ]
}

fn generate_colors(length: usize) -> Vec<RGBColor> {
    let mut colors = HashSet::new();
    let mut rng = rand::thread_rng();
    while colors.len() < length {
        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        let random_color = RGBColor(r, g, b);

        if colors.insert(random_color) {
            println!("Random non-duplicate color {} {} {}", r, g, b);
        }
    }
    colors.into_iter().collect()
}

fn create_random_points(length: usize) -> Vec<delaunay_creator::Point2D> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(-2.0, 2.0);

    (0..length)
        .map(|i| delaunay_creator::Point2D {
            index: i as i64,
            x: rng.sample(range),
            y: rng.sample(range),
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --example 2d_plot [number_of_points]");
        return Ok(());
    }

    let num_points = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number of points. Please input a positive integer.");
            return Ok(());
        }
    };

    let out_file_name = format!("examples/delaunay_2d_{}_points.png", num_points);

    let root = BitMapBackend::new(&out_file_name, (800, 600)).into_drawing_area();

    let random_points = create_random_points(num_points);
    let res = delaunay_creator::bowyer_watson(random_points);

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Delaunay for {} points", num_points),
            "sans-serif",
        )
        .build_cartesian_2d(-2.0..2.0, -2.0..2.0)?;

    let num_triangle = res.len();
    let colors = generate_colors(num_triangle);

    for i in 0..num_triangle {
        let triangle_vertices = triangle_to_vertices(&res[i]);
        let random_color = colors[i];
        chart.draw_series(std::iter::once(Polygon::new(
            triangle_vertices.clone(),
            random_color,
        )))?;
    }
    root.present()?;

    println!("Result has been saved to {}", &out_file_name);

    Ok(())
}
