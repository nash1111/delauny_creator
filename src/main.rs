use std::f64::INFINITY;
use std::collections::HashSet;

use model::{Point2D, Triangle};

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
        a: Point2D { x: -1000.0, y: -1000.0 },
        b: Point2D { x: 1000.0, y: -1000.0 },
        c: Point2D { x: 0.0, y: 1000.0 },
    }
}

// ステップ2a: 指定された点が内部にある三角形を特定する関数
fn find_bad_triangles(triangles: &[Triangle], point: &Point2D) -> Vec<Triangle> {
    triangles
        .iter()
        .filter(|&triangle| is_point_inside_circumcircle(triangle, point))
        .cloned()
        .collect()
}

// ステップ2b: 共有されない辺のリストを作成する関数
fn find_unique_edges(bad_triangles: &[Triangle]) -> HashSet<(Point2D, Point2D)> {
    let mut edges = HashSet::new();

    for triangle in bad_triangles {
        edges.insert((triangle.a, triangle.b));
        edges.insert((triangle.b, triangle.c));
        edges.insert((triangle.c, triangle.a));
    }

    edges
}

// ステップ2c: bad_trianglesを削除する関数
fn remove_bad_triangles(triangles: &mut Vec<Triangle>, bad_triangles: &[Triangle]) {
    triangles.retain(|t| !bad_triangles.contains(t));
}

// ステップ2d: 共有されない辺を使用して新しい三角形を作成し、リストに追加する関数
fn add_new_triangles(triangles: &mut Vec<Triangle>, edges: &HashSet<(Point2D, Point2D)>, point: &Point2D) {
    for edge in edges {
        triangles.push(Triangle {
            a: edge.0,
            b: edge.1,
            c: *point,
        });
    }
}

// ステップ3: スーパートライアングルに接続されている三角形を削除する関数
fn remove_super_triangle_triangles(triangles: &mut Vec<Triangle>, super_triangle: &Triangle) {
    triangles.retain(|t| {
        !t.contains_vertex(super_triangle.a)
            && !t.contains_vertex(super_triangle.b)
            && !t.contains_vertex(super_triangle.c)
    });
}

fn bowyer_watson(points: Vec<Point2D>) -> Vec<Triangle> {
    let mut triangles = vec![];
      // ステップ1: スーパートライアングルを作成
      let super_triangle = create_super_triangle();
      triangles.push(super_triangle);
  
      // ステップ2: 点を追加していく
      for point in &points {
          let bad_triangles = find_bad_triangles(&triangles, point);
          let unique_edges = find_unique_edges(&bad_triangles);
  
          // ステップ2c: bad_trianglesを削除
          remove_bad_triangles(&mut triangles, &bad_triangles);
  
          // ステップ2d: 共有されない辺を使用して新しい三角形を作成し、リストに追加
          add_new_triangles(&mut triangles, &unique_edges, point);
      }
  
      // ステップ3: スーパートライアングルに接続されている三角形を削除
      remove_super_triangle_triangles(&mut triangles, &super_triangle);
  
      triangles
  }
  
  fn is_point_inside_circumcircle(triangle: &Triangle, point: &Point2D) -> bool {
    let a = &triangle.a;
    let b = &triangle.b;
    let c = &triangle.c;

    let ab = a.distance_squared(b);
    let ac = a.distance_squared(c);
    let bc = b.distance_squared(c);

    let p = (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)).abs();
    let s = ab * ac * bc;

    let r_squared = s / (4.0 * p * p);

    let center_x = (a.x + b.x + c.x) / 3.0;
    let center_y = (a.y + b.y + c.y) / 3.0;
    let center = Point2D { x: center_x, y: center_y };

    let distance_squared = point.distance_squared(&center);

    distance_squared < r_squared
}