use luexks_reassembly::shapes::{shape::Shape, shapes::Shapes};

#[derive(Clone)]
pub struct ShapeContainer {
    pub s: Shape,
    pub invert_height_of_mirror: bool,
}

impl ShapeContainer {
    pub fn new(shape: Shape) -> ShapeContainer {
        ShapeContainer {
            s: shape,
            invert_height_of_mirror: false,
        }
    }
}

pub fn restructure_shapes(shapes: Shapes) -> Vec<ShapeContainer> {
    shapes.0.into_iter().map(ShapeContainer::new).collect()
}
