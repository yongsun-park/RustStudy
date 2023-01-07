fn main() {
    // Point struct
    struct Point {
        x: f64,
        y: f64,
    }

    // Polygon struct that has points
    struct Polygon {
        points: Vec<Point>,
    }

    // Implement the Polygon struct
    impl Polygon {
        fn new() -> Self {
            Self { points: Vec::new() }
        }
    }

    // Surface struct that has polygons
    struct Surface {
        polygons: Vec<Polygon>,
    }

    // Implement the Surface struct
    impl Surface {
        fn new() -> Self {
            Self { polygons: Vec::new() }
        }
    }


    // Create a new polygon
    let mut surface = Surface::new();
    let mut poly = Polygon::new();
    // add point to the polygon
    poly.points.push(Point { x: 1.0, y: 2.0 });
    poly.points.push(Point { x: 2.0, y: 1.0 });

    // 10번 반복 하면서 Point를 입력한다
    for i in 0..10 {
        // i가 3일 때, poly를 새로 생성한다.
        if i == 3 {
            poly = Polygon::new();
            surface.polygons.push(poly);
        }
        // i가 4이상이면 poly에 Point를 추가한다.
        if i >= 4 {
            let pt = Point { x: i as f64, y: i as f64 };
            // surface polygons의 마지막 항목의 points에 pt를 추가한다.
            surface.polygons.last_mut().unwrap().points.push(pt);
        }
    }

    // Print the Surface polygons points
    for poly in surface.polygons.iter() {
        for point in poly.points.iter() {
            println!("x: {}, y: {}", point.x, point.y);
        }
    }
}
