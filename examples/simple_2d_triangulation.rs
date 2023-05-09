fn main() {
    
    // Start indexing from 1
    let square = vec![
        delaunay_creator::Point2D { index: 1, x: 0.0, y: 0.0 },
        delaunay_creator::Point2D { index: 2, x: 1.0, y: 0.0 },
        delaunay_creator::Point2D { index: 3, x: 0.0, y: 1.0 },
        delaunay_creator::Point2D { index: 4, x: 1.0, y: 1.0 },
    ];
    let res = delaunay_creator::bowyer_watson(square);
    println!("{:?}", res);
}
