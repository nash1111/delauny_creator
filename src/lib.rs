pub use model::{Edge, Point2D, Triangle};

mod model;

pub fn bowyer_watson(points: Vec<Point2D>) -> Vec<Triangle> {
    let mut triangulation: Vec<Triangle> = Vec::new();
    let super_triangle = create_super_triangle(&points);
    triangulation.push(super_triangle);

    for point in points {
        let mut bad_triangles: Vec<Triangle> = Vec::new();
        for triangle in &triangulation {
            let circumcircle = triangle.generate_circumcircle();
            if circumcircle.point_in_circle(&point) {
                bad_triangles.push(*triangle);
            }
        }

        let mut polygon: Vec<Edge> = Vec::new();
        for triangle in &bad_triangles {
            let edges = triangle.edges();
            let bad_triangles_without_triangle: Vec<Triangle> = bad_triangles
                .iter()
                .filter(|t| t != &triangle)
                .cloned()
                .collect();
            for edge in edges {
                if !edge_is_shared_by_triangles(&edge, &bad_triangles_without_triangle) {
                    polygon.push(edge);
                }
            }
        }

        for bad_triangle in &bad_triangles {
            triangulation.retain(|triangle| triangle != bad_triangle);
        }

        for edge in &polygon {
            let new_tri = retriangulate(&edge, &point);
            triangulation.push(new_tri);
        }
    }
    remove_triangles_with_vertices_from_super_triangle(&mut triangulation, &super_triangle)
}

fn create_super_triangle(points: &Vec<Point2D>) -> Triangle {
    if points.is_empty() {
        panic!("The input points vector should not be empty.");
    }

    let mut min_x = points[0].x;
    let mut max_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;

    for point in points {
        min_x = f64::min(min_x, point.x);
        max_x = f64::max(max_x, point.x);
        min_y = f64::min(min_y, point.y);
        max_y = f64::max(max_y, point.y);
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    let a = Point2D {
        x: min_x - height,
        y: min_y - width,
    };
    let b = Point2D {
        x: max_x + height,
        y: min_y - width,
    };
    let c = Point2D {
        x: min_x + 0.5 * width,
        y: max_y + width,
    };

    Triangle { a, b, c }
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
