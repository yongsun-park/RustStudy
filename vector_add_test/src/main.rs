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

    // string polygons
    let strs = "
    SB
        PB
            PT 350 130
            PT 350 170
            PT 450 170
        PE 
        PB
            PT 370 150
            PT 370 160
            PT 430 160
        PE
    SE";

    let mut surface = Surface::new();
    let mut poly:Polygon = Polygon::new();

    let lines: Vec<&str> = strs.lines().collect();
    for line in lines.iter().map(|s| s.trim()) {
        
        if line == "SB" {
            surface = Surface::new();
        } else if line == "PE" {
            surface.polygons.push(poly);
            poly = Polygon::new();
        } else if line.starts_with("PT") {
            let mut parts = line.split_whitespace();
            parts.next(); // skip "PT"
            let x: f64 = parts.next().unwrap().parse().unwrap();
            let y: f64 = parts.next().unwrap().parse().unwrap();
            let pt = Point { x, y };

            poly.points.push(pt); // <-- Error
            // surface.polygons.last_mut().unwrap().points.push(pt);
        }
    }

    // Print the Surface polygons points
    for poly in surface.polygons.iter() {
        println!("PB");
        for point in poly.points.iter() {
            println!("\tx: {}, y: {}", point.x, point.y);
        }
        println!("PE");
    }
}
