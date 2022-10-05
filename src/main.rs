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

fn update(app: &App, _model: &mut Model, _update: Update) {
    
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background().color(PLUM);

    draw.ellipse().color(YELLOW).x_y(0.0, 0.0);

    let planets = &model::PLANETS;
    for planet in planets {
        let time = (app.time / planet.year_in_days) * 100.0;

        let x = planet.orbit_radius * time.cos() + 0.0;
        let y = planet.orbit_radius * time.sin() + 0.0;

        draw.ellipse().color(planet.color).radius(planet.planet_radius).x_y(x, y);
    }

    draw.to_frame(app, &frame).unwrap();
}