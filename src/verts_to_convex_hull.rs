// https://github.com/manylegged/outlaws-core/blob/24ccab40ecbd978bcdb333a674b7f77c73827eda/Geometry.cpp#L159

use std::cmp::Ordering;

use egui::Pos2;

pub fn verts_to_convex_hull(mut verts: Vec<Pos2>) -> Vec<Pos2> {
    let original_verts = verts.clone();
    if verts.len() <= 3 {
        return verts;
    }

    let mut min_y = verts[0].y;
    let mut min_y_idx = 0;

    verts.iter().enumerate().for_each(|(idx, vert)| {
        if vert.y < min_y {
            min_y = vert.y;
            min_y_idx = idx;
        }
    });

    verts.swap(0, min_y_idx);

    let first = verts[0];
    verts[1..].sort_by(|a, b| match Orientation::from((first, a, b)) {
        Orientation::Clockwise => Ordering::Less,
        Orientation::Anticlockwise => Ordering::Greater,
        Orientation::Collinear => {
            let a_dist_squared = (a.x - first.x).powi(2) + (a.y - first.y).powi(2);
            let b_dist_squared = (b.x - first.x).powi(2) + (b.y - first.y).powi(2);
            a_dist_squared
                .partial_cmp(&b_dist_squared)
                .unwrap_or(Ordering::Equal)
        }
    });
    let mut verts = verts.into_iter().map(|vert| Some(vert)).collect::<Vec<_>>();
    const EPSILON: f32 = 1e-06 * 1e-06;
    for (curr, next) in (0..verts.len() - 1).zip(1..verts.len()) {
        let (Some(a), Some(b)) = (verts[curr], verts[next]) else {
            continue;
        };
        let square_difference = (a - b).length_sq();
        if square_difference < EPSILON {
            verts[curr] = None;
        }
    }
    let mut verts = verts.into_iter().flatten().collect::<Vec<_>>();
    let n = verts.len();
    println!("n={}", n);
    dbg!(&verts);
    let mut i = 1;
    let mut m = 1;
    while i < n {
        println!("m={}\ti={}", m, i);
        while Orientation::from((verts[m - 1], &verts[m], &verts[i])) == Orientation::Clockwise {
            println!("m={}\ti={}", m, i);
            if m > 1 {
                m -= 1;
            } else if i == n - 1 {
                return verts;
            } else {
                i += 1;
            }
        }
        m += 1;
        println!("m={}\ti={}", m, i);
        verts.swap(m, i);
        dbg!(&verts);
        i += 1;
    }
    println!("Error in verts to convex hull");
    original_verts
}

#[derive(PartialEq, Eq)]
enum Orientation {
    Clockwise,
    Anticlockwise,
    Collinear,
}

impl From<(Pos2, &Pos2, &Pos2)> for Orientation {
    fn from((a, b, c): (Pos2, &Pos2, &Pos2)) -> Self {
        // Signed Area Determinant
        let v = (a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y));
        println!("orientation get {}", v);
        if v < 0.0 {
            Orientation::Clockwise
        } else if v > 0.0 {
            Orientation::Anticlockwise
        } else {
            Orientation::Collinear
        }
    }
}

impl From<Orientation> for Ordering {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Clockwise => Ordering::Less,
            Orientation::Anticlockwise => Ordering::Greater,
            Orientation::Collinear => Ordering::Equal,
        }
    }
}
