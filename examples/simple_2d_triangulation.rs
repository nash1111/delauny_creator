fn main() {
    let square = vec![
        delaunay_creator::Point2D { x: 0.0, y: 0.0 },
        delaunay_creator::Point2D { x: 1.0, y: 0.0 },
        delaunay_creator::Point2D { x: 0.0, y: 1.0 },
        delaunay_creator::Point2D { x: 1.0, y: 1.0 },
    ];
    let res = delaunay_creator::bowyer_watson(square);
    println!("{:?}", res);
}
