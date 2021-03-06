use sfml::system::vector2;
use sfml::traits::Drawable;
use sfml::graphics::{Font, Text, Color, RenderTarget};
use sfml::graphics;
use time;

pub struct Fps<'a> {
    delta:     f64,
    last_time: f64,
    fps:       Text<'a>
}

impl<'a> Fps<'a> {
    pub fn new(font: &'a Font) -> Fps<'a> {
        let mut txt = Text::new().unwrap();
        txt.set_font(font);
        txt.set_position(&vector2::Vector2f { x: 0.0, y: 0.0 });
        txt.set_color(&Color::new_rgb(255, 255, 255));

        Fps {
            delta:     0.0,
            last_time: 0.0,
            fps:       txt
        }
    }

    pub fn reset(&mut self) {
        self.last_time = time::precise_time_s();
    }

    pub fn register_delta(&mut self) {
        self.delta = self.elapsed_seconds()
    }

    pub fn elapsed_seconds(&self) -> f64 {
        time::precise_time_s() - self.last_time
    }

    pub fn draw_registered(&mut self, rw: &mut graphics::RenderWindow) {
        let elapsed = self.delta;

        let v = rw.get_view();

        self.fps.set_position(&rw.map_pixel_to_coords(&vector2::Vector2i { x: 0, y : 0 }, &v));
        self.fps.set_string(&elapsed.to_string()[..]);
        rw.draw(&self.fps);
    }
}
