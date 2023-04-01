use std::collections::HashSet;
use std::f64::INFINITY;

use model::{Edge, Point2D, Triangle};

mod model;

fn main() {
    println!("Hello, world!");
    // define points as a vector of Point2D
    let points = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 0.0 },
        Point2D { x: 0.0, y: 1.0 },
        Point2D { x: 1.0, y: 1.0 },
    ];
    let res = bowyer_watson(points);
    dbg!(res);
}

// ステップ1: スーパートライアングルを作成する関数
fn create_super_triangle() -> Triangle {
    Triangle {
        a: Point2D {
            x: -100.0,
            y: -100.0,
        },
        b: Point2D {
            x: 100.0,
            y: -100.0,
        },
        c: Point2D { x: 0.0, y: 100.0 },
    }
}

fn bowyer_watson(points: Vec<Point2D>) -> Vec<Triangle> {
    let mut triangulation: Vec<Triangle> = Vec::new();

    triangulation.push(create_super_triangle());

    let mut i = 0;
    for point in points {
        println!("index {:?}", i);
        let mut bad_triangles: Vec<Triangle> = Vec::new();
        for triangle in &triangulation {
            let circumcircle = triangle.generate_circumcircle_fin();
            println!("{:?}", circumcircle);
            if circumcircle.point_in_circle(&point) {
                println!("{:?} is in circumcircle of {:?}", point, triangle);
                bad_triangles.push(*triangle);
            }
        }
        println!("bad triangles 51 {:?}", bad_triangles);

        // find the boundary of the polygonal hole
        let mut polygon: Vec<Edge> = Vec::new();
        for triangle in &bad_triangles {
            let edges = triangle.edges();
            // bad_trianbles without triangle
            // TODO: ほんとに他の三角だけを見に行ってる？
            println!("bad triangles {:?}", &bad_triangles);
            println!("triangle {:?}", triangle);
            let bad_triangles_without_triangle: Vec<Triangle> = bad_triangles
                .iter()
                .filter(|t| t != &triangle)
                .cloned()
                .collect();
            println!(
                "bad triangles without triangle {:?}",
                &bad_triangles_without_triangle
            );
            for edge in edges {
                println!("edge {:?}", edge);
                println!("polygon {:?}", polygon);
                // TODO
                // FIX
                // 他の三角だけを見に行く必要！！！
                if !edge_is_shared_by_triangles(&edge, &bad_triangles_without_triangle) {
                    println!("push! {:?} is not shared by bad triangles", edge);
                    polygon.push(edge);
                    println!("polygon {:?}", polygon)
                } else {
                    println!("no push! {:?} is shared by bad triangles", edge);
                }
            }
        }

        // remove badTriangles from triangulation
        for bad_triangle in &bad_triangles {
            println!("-----start remove triangle {:?}", triangulation);
            triangulation.retain(|triangle| triangle != bad_triangle);
            println!("-----end remove triangle {:?}", triangulation);
        }

        // adding new triangles to the triangulation
        for edge in &polygon {
            let new_tri = retriangulate(&edge, &point);
            dbg!(&new_tri);
            triangulation.push(new_tri);
        }

        println!("bad triangles {:?}", &bad_triangles);
        println!("triangulation {:?}", &triangulation);
        //println!("polygon {:?}", &polygon);
        i += 1;
        if i == 9 {
            let res = remove_triangles_with_vertices_from_super_triangle(
                &mut triangulation,
                &create_super_triangle(),
            );
            println!("res {:?}", res);
            println!("final polygon {:?}", &polygon);
            panic!("stop");
            //
        }
    }

    // 一個しか正解のTriangleなかった
    // なんで？
    //　しかしスーパートライアングルの除去だけはうまく行ってそう
    //triangulation
    remove_triangles_with_vertices_from_super_triangle(&mut triangulation, &create_super_triangle())
}

fn edge_is_shared_by_triangles(edge: &Edge, triangles: &Vec<Triangle>) -> bool {
    for triangle in triangles {
        let edges_of_triangle = triangle.edges();
        for edge_of_triangle in edges_of_triangle {
            if edge_of_triangle == *edge {
                return true;
            }
            if edge_of_triangle.reverse() == *edge {
                return true;
            }
        }
    }
    false
}

// we need fn(triangle: &Triangle, point: &Point2D) -> Triangle
// re-triangulate the polygonal hole
fn retriangulate(edge: &Edge, point: &Point2D) -> Triangle {
    Triangle {
        a: edge.start,
        b: edge.end,
        c: *point,
    }
}

fn triangle_contains_vertex_from_super_triangle(
    triangle: &Triangle,
    super_triangle: &Triangle,
) -> bool {
    let super_triangle_vertices = super_triangle.vertices();
    let triangle_vertices = triangle.vertices();
    for super_triangle_vertex in super_triangle_vertices {
        if super_triangle_vertex == triangle_vertices[0] {
            return true;
        }
        if super_triangle_vertex == triangle_vertices[1] {
            return true;
        }
        if super_triangle_vertex == triangle_vertices[2] {
            return true;
        }
    }
    false
}

fn remove_triangles_with_vertices_from_super_triangle(
    triangles: &Vec<Triangle>,
    super_triangle: &Triangle,
) -> Vec<Triangle> {
    let mut res: Vec<Triangle> = Vec::new();
    for triangle in triangles {
        if !triangle_contains_vertex_from_super_triangle(triangle, super_triangle) {
            res.push(*triangle);
        }
    }
    res
}
