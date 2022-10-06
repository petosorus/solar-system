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

    const sun_size: f32 = 1392684.0;

    let r = model::MAX_ORBIT / sun_size;
    let length = boundary.top();

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
        let time = (app.time / planet.year_in_days) * 100.0;

        // let x = (planet.orbit_radius * orbit_scale) * time.cos();
        // let y = (planet.orbit_radius * orbit_scale) * time.sin();

        let ratio = sun_size / planet.orbit_radius;

        let x = ((length * ratio.log10()) / r.log10()) * time.cos();
        let y = ((length * ratio.log10()) / r.log10()) * time.sin();

        draw.ellipse().color(planet.color).radius(planet.planet_radius / planet_scale).x_y(x, y);
    }

    draw.to_frame(app, &frame).unwrap();
}