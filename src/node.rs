use nannou::prelude::Vec2;

pub struct Node {
    pub pos: Vec2,
    pub parent: Option<usize>,
}

impl Node {
    pub fn new(pos: Vec2) -> Self {
        Node { pos, parent: None }
    }
}
