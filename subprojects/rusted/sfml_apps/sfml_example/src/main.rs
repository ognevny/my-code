use sfml::{
    graphics::{
        CircleShape, Color, FloatRect, RenderTarget, RenderWindow, Shape, Transformable, View,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (200, 200),
        "blazingly fast, memory-safe sfml window",
        Style::RESIZE | Style::CLOSE,
        &Default::default(),
    );
    let mut shape = CircleShape::new(100., 30);
    shape.set_fill_color(Color::GREEN);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::Resized { width, height } => {
                    let view = View::from_rect(FloatRect::new(0., 0., width as f32, height as f32));
                    let radius = width.min(height) as f32 / 2.;
                    shape.set_radius(radius);
                    shape.set_origin(Vector2f::new(radius, radius));
                    shape.set_scale(Vector2f::new(width as f32 / height as f32, 1.));
                    shape.set_position(Vector2f::new(width as f32 / 2., height as f32 / 2.));
                    window.set_view(&view);
                },
                _ => (),
            }
        }
        window.clear(Color::rgb(0, 0, 0));
        window.draw(&shape);
        window.display();
    }
}
