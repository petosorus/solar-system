use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(PLUM);

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    let boundary = app.window_rect();

    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    let x = 300.0 * app.time.cos() + 0.0;
    let y = 200.0 * app.time.sin() + 0.0;

    draw.ellipse().color(YELLOW).x_y(0.0, 0.0);
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    // draw.ellipse().color(STEELBLUE).x_y(0.0, 0.0);

    draw.to_frame(app, &frame).unwrap();
}