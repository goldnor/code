use crate::{hittable::Hittable, prelude::*};

pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: i32,

    /// Rendered image height
    image_height: i32,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}

impl Camera {
    pub fn with_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;

        self
    }

    pub fn with_image_width(mut self, image_width: i32) -> Self {
        self.image_width = image_width;

        self
    }

    pub fn render(&mut self, world: &impl Hittable) -> std::io::Result<()> {
        self.initialize();

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64) * self.pixel_delta_u
                    + (j as f64) * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(r, world);
                write_color(std::io::stdout(), pixel_color)?;
            }
        }
        info!("Done.");

        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = {
            let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
            if image_height < 1 { 1 } else { image_height }
        };

        self.center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: Ray, world: &impl Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
