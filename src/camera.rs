use crate::{hittable::Hittable, prelude::*};

pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: i32,
    // Count of random samples for each pixel
    pub samples_per_pixel: i32,
    // Maximum number of ray bounces into scene
    pub max_depth: i32,
    // Vertical view angle (field of view)
    pub vfov: f64,
    /// Point camera is looking from
    pub lookfrom: Point3,
    /// Point camera is looking at
    pub lookat: Point3,
    /// Camera-relative "up" direction
    pub vup: Vec3,
    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,
    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,

    /// Rendered image height
    image_height: i32,
    // Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    /// Camera center
    center: Point3,
    /// Location of pixel 0, 0
    pixel00_loc: Point3,
    /// Offset to pixel to the right
    pixel_delta_u: Vec3,
    /// Offset to pixel below
    pixel_delta_v: Vec3,
    /// Camera frame basis vector - right
    u: Vec3,
    /// Camera frame basis vector - up
    v: Vec3,
    /// Camera frame basis vector - opposite view direction
    w: Vec3,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Point3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
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

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;

        self
    }

    pub fn with_max_depth(mut self, max_depth: i32) -> Self {
        self.max_depth = max_depth;

        self
    }

    pub fn with_vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;

        self
    }

    pub fn with_lookfrom(mut self, lookfrom: Point3) -> Self {
        self.lookfrom = lookfrom;

        self
    }

    pub fn with_lookat(mut self, lookat: Point3) -> Self {
        self.lookat = lookat;

        self
    }

    pub fn with_vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;

        self
    }

    pub fn with_defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;

        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;

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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(r, self.max_depth, world);
                }
                write_color(std::io::stdout(), self.pixel_samples_scale * pixel_color)?;
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

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Determine viewport dimensions.
        let theta = self.vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * f64::tan((self.defocus_angle / 2.0).to_radians());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            rand::random::<f64>() - 0.5,
            rand::random::<f64>() - 0.5,
            0.0,
        )
    }

    fn _sample_disk(radius: f64) -> Vec3 {
        // Returns a random point in the unit (radius 0.5) disk centered at the origin.
        radius * random_in_unit_disk()
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();

        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(r: Ray, depth: i32, world: &impl Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, INFINITY)) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, rec.clone()) {
                return attenuation * Self::ray_color(scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
