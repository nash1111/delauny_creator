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
    match points.is_empty() {
        true => panic!("The input points vector should not be empty."),
        false => {}
    }

    let index = 0;
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for point in points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let margin = 100.0;

    let a = Point2D {
        index,
        x: min_x - margin,
        y: min_y - margin,
    };
    let b = Point2D {
        index,
        x: max_x + margin,
        y: min_y - margin,
    };
    let c = Point2D {
        index,
        x: (min_x + max_x) / 2.0,
        y: max_y + margin,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bowyer_watson() {
        let square = vec![
            Point2D {
                x: 0.0,
                y: 0.0,
                index: 0,
            },
            Point2D {
                x: 1.0,
                y: 0.0,
                index: 1,
            },
            Point2D {
                x: 0.0,
                y: 1.0,
                index: 2,
            },
            Point2D {
                x: 1.0,
                y: 1.0,
                index: 3,
            },
        ];
        let result = bowyer_watson(square);
        assert_eq!(result.len(), 2);
    }
}
