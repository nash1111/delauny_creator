use std::f64::INFINITY;
use std::collections::HashSet;

use model::{Point2D, Triangle, Edge};

mod model;

fn main() {
    println!("Hello, world!");
    // define points as a vector of Point2D
    let points = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 0.0 },
        Point2D { x: 0.0, y: 1.0 },
        Point2D { x: 1.0, y: 1.0 },
        Point2D { x: 1.0, y: 2.0 },
        Point2D { x: 1.0, y: 3.0 },
    ];
    let res = bowyer_watson(points);
    dbg!(res);
}

// ステップ1: スーパートライアングルを作成する関数
fn create_super_triangle() -> Triangle {
    Triangle {
        a: Point2D { x: -INFINITY, y: -INFINITY },
        b: Point2D { x: INFINITY, y: -INFINITY },
        c: Point2D { x: 0.0, y: INFINITY },
    }
}

fn bowyer_watson(points: Vec<Point2D>) -> Vec<Triangle> {
    let mut triangulation:Vec<Triangle> = Vec::new();
    // TODO
    // スーパートライアングルは無限使って良いのか？
    triangulation.push(create_super_triangle());

    for point in points {
        let mut bad_triangles:Vec<Triangle> = Vec::new();
        for triangle in &triangulation {
            // TODO
            // circumcircle_contains()が怪しい
            if triangle.circumcircle_contains(&point) {
                bad_triangles.push(*triangle);
            }
        }

        // find the boundary of the polygonal hole
        let mut polygon:Vec<Edge> = Vec::new();
        for triangle in &bad_triangles {
            let edges = triangle.edges();
            for edge in edges {
                if edge_is_not_shared_by_trianbles(&edge, &bad_triangles) {
                    polygon.push(edge);
                }
            }
        }

        // remove badTriangles from triangulation
        // TODO retain 見直し
        for bad_triangle in &bad_triangles {
            triangulation.retain(|triangle| triangle != bad_triangle);
        }

        for edge in polygon {
            triangulation.push(retriangulate(&edge, &point));
        }
    }
    remove_triangles_with_vertices_from_super_triangle(&mut triangulation, &create_super_triangle())
}

fn edge_is_not_shared_by_trianbles(edge: &Edge, triangles: &Vec<Triangle>) -> bool {
    for triangle in triangles {
        let edges_of_triangle = triangle.edges();
        for edge_of_triangle in edges_of_triangle {
            if edge_of_triangle == *edge {
                return false;
            }
            if edge_of_triangle.reverse() == *edge {
                return false;
            }
        }

    }
    true
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

fn triangle_contains_vertex_from_super_triangle(triangle: &Triangle, super_triangle: &Triangle) -> bool {
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

fn remove_triangles_with_vertices_from_super_triangle(triangles: &Vec<Triangle>, super_triangle: &Triangle) -> Vec<Triangle> {
    let mut res:Vec<Triangle> = Vec::new();
    for triangle in triangles {
        if !triangle_contains_vertex_from_super_triangle(triangle, super_triangle) {
            res.push(*triangle);
        }
    }
    res
}