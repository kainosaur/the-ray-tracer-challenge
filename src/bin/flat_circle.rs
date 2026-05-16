use ray_tracer::{features::{canvas::Canvas, operators::Normalize, rays::Ray, tuple::{Color, Point}}, objects::spheres::Sphere};
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels: f32 = 100.;
    let pixel_size: f32 = wall_size / canvas_pixels;
    let half = wall_size / 2.;
    let mut canvas = Canvas::new(canvas_pixels.round() as usize, canvas_pixels.round() as usize);
    let color = Color::new(1., 0., 0.);
    let shape = Sphere::new();

    // For each row of pixels in the canvas
    for y in 0..canvas_pixels.round() as usize {
        // Compute the world y coordinate (top = +half, bottom = -half)
        let world_y = half - pixel_size * y as f32;
        // For each pixel in the row
        for x in 0..canvas_pixels.round() as usize {
            // for each pixel in the row
            let world_x = -half + pixel_size * x as f32;

            // describe the point on the wall that the ray will target
            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersect_sphere = shape.intersect(ray);

            let hit = intersect_sphere.hit();

            match hit {
                Some(_) => canvas.write_pixel(x, y, color),
                None => {}
            }
        }
    }

    let mut file = File::create("Circle_Flat.PPM")?;
    file.write_all(canvas.create_file_string().as_bytes())?;
    Ok(())
}