use nannou::prelude::*;
use node::Node;
use rand::{thread_rng, Rng};
use rayon::{
    iter::ParallelIterator,
    prelude::{IndexedParallelIterator, IntoParallelRefMutIterator},
};

mod node;

const MAX_LINE_SIZE: f32 = 20.0;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    root: Vec<Node>,
    redraw: bool,
}

impl Model {
    fn new() -> Self {
        Model {
            root: vec![Node::new(Vec2::new(0.0, 0.0))],
            redraw: false,
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1280, 720)
        .title("RRT Plot")
        .view(view)
        .build()
        .unwrap();

    Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let size = app.main_window().inner_size_pixels();
    let (width, height) = (size.0 as f32, size.1 as f32);
    let (x, y) = (width / 2.0, height / 2.0);

    model.redraw = false;
    for (press, pos) in app.mouse.buttons.pressed() {
        if press == MouseButton::Left || model.root.len() > (width as usize * height as usize) {
            model.redraw = true;
            model.root.clear();
            model.root.push(Node::new(pos));
            return;
        }
    }

    // Pick random coordinate. Find node closest to it.
    // Add child to it that is move_by amount closer to coordinate.
    let random_pos = Vec2::new(thread_rng().gen_range(-x..x), thread_rng().gen_range(-y..y));
    let (index, closest) = model
        .root
        .par_iter_mut()
        .enumerate()
        .min_by(|(_, x), (_, y)| {
            x.pos
                .distance(random_pos)
                .total_cmp(&y.pos.distance(random_pos))
        })
        .unwrap();

    let move_by = closest.pos.distance(random_pos).min(MAX_LINE_SIZE);
    let direction = (random_pos - closest.pos).normalize_or_zero();

    let child_pos = closest.pos + (direction * move_by);

    let mut node = Node::new(child_pos);
    node.parent = Some(index);

    model.root.push(node);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();

    if model.redraw {
        frame.clear(nannou::color::BLACK);
        draw.to_frame(app, &frame).unwrap();
        return;
    }

    let last = model.root.last().expect("Vec was empty in fn view");
    if let Some(start) = last.parent {
        make_line(&mut draw, model.root[start].pos, last.pos);
    }
    make_ellipse(&mut draw, last.pos);

    draw.to_frame(app, &frame).unwrap();
}

fn make_ellipse(draw: &mut Draw, pos: Vec2) {
    draw.ellipse()
        .xy(pos)
        .w_h(5.0, 5.0)
        .color(nannou::color::RED);
}

fn make_line(draw: &mut Draw, start: Vec2, end: Vec2) {
    draw.line()
        .start(start)
        .end(end)
        .stroke_weight(0.4)
        .color(nannou::color::RED);
}
