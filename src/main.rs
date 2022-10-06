use nannou::prelude::*;
mod model;

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

fn update(_app: &App, _model: &mut Model, _update: Update) {
    
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(PLUM);

    // figure orbit scale out
    let boundary = app.window_rect();
    let height = boundary.top() - boundary.bottom();
    let orbit_scale = (height / model::MAX_ORBIT) / 2.0;

    // figure planet scale
    let smallest_planet = model::SMALLEST_PLANET;
    let smallest_drawing = 1.0;
    let planet_scale = smallest_planet / smallest_drawing;

    // sun
    // draw.ellipse().color(YELLOW).radius(1392684.0 / planet_scale).x_y(0.0, 0.0);
    draw.ellipse().color(YELLOW).x_y(0.0, 0.0);

    // draw the system
    let planets = &model::PLANETS;
    for planet in planets {
        let time = (app.time / planet.year_in_days) * 1000.0;

        let x = (planet.orbit_radius * orbit_scale) * time.cos();
        let y = (planet.orbit_radius * orbit_scale) * time.sin();

        draw.ellipse().color(planet.color).radius(planet.planet_radius / planet_scale).x_y(x, y);
    }

    draw.to_frame(app, &frame).unwrap();
}