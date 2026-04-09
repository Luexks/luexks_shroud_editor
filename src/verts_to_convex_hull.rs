// https://github.com/manylegged/outlaws-core/blob/24ccab40ecbd978bcdb333a674b7f77c73827eda/Geometry.cpp#L159

use std::{cmp::Ordering, collections::VecDeque};

use egui::Pos2;

pub fn verts_to_convex_hull(mut verts: Vec<Pos2>, is_vanilla: bool) -> Vec<Pos2> {
    let original_verts = verts.clone();
    if verts.len() <= 3 {
        return verts;
    }

    if is_vanilla {
        let mut min_y = verts[0].y;
        let mut min_y_idx = 0;

        verts.iter().enumerate().for_each(|(idx, vert)| {
            if vert.y < min_y {
                min_y = vert.y;
                min_y_idx = idx;
            }
        });
        verts.swap(0, min_y_idx);
    } else {
        let mut min_x = verts[0].x;
        let mut min_x_idx = 0;
        let mut min_y = verts[0].y;

        verts.iter().enumerate().for_each(|(idx, vert)| {
            if vert.x < min_x {
                min_x = vert.x;
                min_x_idx = idx;
            } else if vert.x == min_x && vert.y < min_y {
                min_y = vert.y;
                min_x_idx = idx;
            }
        });
        verts.swap(0, min_x_idx);
    }

    let first = verts[0];
    // println!("##############");
    const EPSILON: f32 = 1e-06 * 1e-06;
    // let mut verts = verts[1..];
        // .into_iter()
        // .map(|vert| (*vert, dbg!((vert.y - first.y).atan2(vert.x - first.x) / std::f32::consts::PI * 180.)))
        // .sorted_by(|(_, angle_1), (_, angle_2)| angle_1.partial_cmp(angle_2).unwrap_or(Ordering::Equal))
        // .dedup_by(|(_, angle_1), (_, angle_2)| angle_1 != angle_2)
        // .collect::<Vec<_>>();
    // verts.sort_by(|(a, _), (b, _)| {
    // let mut verts = verts.into_iter().enumerate().map(|(i, vert)| (vert, i)).collect::<Vec<_>>();
    // verts[1..].sort_by(|(a, a_i), (b, b_i)| {
    verts.remove(0);
    verts.sort_by(|a, b| {
        // match Orientation::signed_area_determinant((first, a, b)) {
        //     Orientation::Clockwise => Ordering::Less,
        //     Orientation::Anticlockwise => Ordering::Greater,
        //     Orientation::Collinear => {
        //         let a_dist_squared = (a.x - first.x).powi(2) + (a.y - first.y).powi(2);
        //         let b_dist_squared = (b.x - first.x).powi(2) + (b.y - first.y).powi(2);
        //         a_dist_squared
        //             .partial_cmp(&b_dist_squared)
        //             .unwrap_or(Ordering::Equal)
        //         // a_dist_squared
        //         //     .total_cmp(&b_dist_squared)
        //         // a_i.cmp(b_i)
        //     }
        // }
        let angle_a = (a.x - first.x).atan2(a.y - first.y);
        let angle_b = (b.x - first.x).atan2(b.y - first.y);
        angle_a.total_cmp(&angle_b)
    });
    // let mut verts = verts.into_iter().dedup_by(|a, b| {
    //     let angle_a = (a.x - first.x).atan2(a.y - first.y);
    //     let angle_b = (b.x - first.x).atan2(b.y - first.y);
    //     (angle_a - angle_b).abs() < EPSILON
    // } ).collect::<Vec<_>>();
    verts.insert(0, first);
    let mut verts = verts
        .into_iter()
        // .map(|(vert, _)| vert)
        .map(Some)
        .collect::<Vec<_>>();
    // verts.insert(0, Some(first));
    for (curr, next) in (1..verts.len() - 1).zip(2..verts.len()) {
        let (Some(a), Some(b)) = (verts[curr], verts[next]) else {
            continue;
        };
        let square_difference = (a - b).length_sq();
        if square_difference < EPSILON {
            verts[curr] = None;
            continue;
        }
        let angle_a = (a.x - first.x).atan2(a.y - first.y);
        let angle_b = (b.x - first.x).atan2(b.y - first.y);
        let a_dist_squared = (a.x - first.x).powi(2) + (a.y - first.y).powi(2);
        let b_dist_squared = (b.x - first.x).powi(2) + (b.y - first.y).powi(2);
        if (angle_a - angle_b).abs() < EPSILON {
            if a_dist_squared < b_dist_squared {
                verts[curr] = None;
            } else {
                // verts[next] = None;
                verts[next] = verts[curr];
                verts[curr] = None;
            }
        }
    }
    let mut verts = verts.into_iter().flatten().collect::<VecDeque<_>>();
    if verts.len() < 3 {
        return original_verts;
    }
    let mut convex_verts = Vec::new();
    convex_verts.push(verts.pop_front().unwrap());
    convex_verts.push(verts.pop_front().unwrap());
    convex_verts.push(verts.pop_front().unwrap());
    // while !verts.is_empty() {
    loop {
        if convex_verts.len() < 3 {
            convex_verts.push(verts.pop_front().unwrap());
        } else {
            let a = convex_verts.len() - 3;
            let b = convex_verts.len() - 2;
            let c = convex_verts.len() - 1;
            let orientation = Orientation::z_of_cross_product((
                convex_verts[a],
                &convex_verts[b],
                &convex_verts[c],
            ));
            if orientation == Orientation::Anticlockwise {
                convex_verts.remove(b);
            } else if verts.is_empty() {
                break;
            } else {
                convex_verts.push(verts.pop_front().unwrap());
            }
        }
    }
    // dbg!(convex_verts)
    convex_verts
}

#[derive(PartialEq, Eq, Debug)]
enum Orientation {
    Clockwise,
    Anticlockwise,
    Collinear,
}

impl Orientation {
    // fn signed_area_determinant((a, b, c): (Pos2, &Pos2, &Pos2)) -> Self {
    //     let v = (a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y));
    //     // println!("orientation get {}", v);
    //     if v < 0.0 {
    //         Orientation::Clockwise
    //     } else if v > 0.0 {
    //         Orientation::Anticlockwise
    //     } else {
    //         Orientation::Collinear
    //     }
    // }

    fn z_of_cross_product((a, b, c): (Pos2, &Pos2, &Pos2)) -> Self {
        let v = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
        // println!("orientation get {}", v);
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
