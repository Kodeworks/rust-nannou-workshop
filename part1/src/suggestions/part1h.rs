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

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();

    draw.background()
        .color(CYAN);

    let win = app.window_rect();

    let circle_radius_a = 50.0;
    let pos_a = vec2( (app.time * 0.5).sin() * win.w() * 0.25, 0.0);

    draw.ellipse()
        .xy(pos_a)
        .radius(circle_radius_a)
        .color(MAGENTA);


    let circle_radius_b = circle_radius_a / 2.0;
    let pos_b = vec2(
        (3.0 * app.time).sin() * 100.0,
        (3.0 * app.time).cos() * 100.0,
        );

    draw.xy(pos_a) // move the draw context to a different position temporarily
        .ellipse()
        .xy(pos_b)
        .radius(circle_radius_b)
        .color(ORANGE);

    //Draw the basic structure of the building
    let unit_size = 50.0;
    let num_floors = 4;
    let windows_per_floor = 2;
    //Convert the i32 primitive type to f32 using the
    // `as` keyword. This way of converting types is only
    // applicable for primitive types such as f32, u32
    // etc.
    let building_rect = Rect::from_w_h(
        unit_size * windows_per_floor as f32, 
        unit_size * num_floors as f32
        )
        .mid_bottom_of(win); // Uses the Rect api to position rectangle relative to 
                             //  the window Rect.

    draw.rect()
        .xy(building_rect.xy()) // use the rect properties directly on the drawing
        .wh(building_rect.wh())
        .color(GREY);

    //Just a triangle to check that it draw smack in the center, i.e. it hasn't been
    // affected by changing the position of the drawing context in the rect drawing 
    // section above.
    draw.tri()
        .x_y(0.0, 0.0)
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}
