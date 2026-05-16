use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;

use ray_tracer::features::canvas::Canvas;
use ray_tracer::features::matrices::{Axis, Matrix4x4};
use ray_tracer::features::tuple::{Color, Point};

fn main() -> std::io::Result<()> {
    
    let mut canvas = Canvas::new(250, 250);
    let drawn_point = Point::new(0.,0.,0.);
    let origin = Point::new(canvas.width as f32 / 2., canvas.height as f32 / 2., 0.);

    let scale = Matrix4x4::scaling(100., 100., 0.);
    let translation_x = Matrix4x4::translation(1., 0., 0.);
    // write pixels around circle 
    let red = Color::new(1.,1.,1.);

    for i in 0..13 {
        let current_rad = i as f32 * PI / 6.;
        let rotation = Matrix4x4::rotation(Axis::Z, current_rad);
        let new_point = (scale * rotation * translation_x * drawn_point).as_point();
        println!("x: {}, y: {}, z: {}",new_point.x() + origin.x(),  new_point.y() + origin.y(), new_point.z());
        canvas.write_pixel
        (
            (new_point.x().round() + origin.x().round()) as usize, 
            (new_point.y().round() + origin.y().round()) as usize,
            red
        );
    }

    let mut file = File::create("Circle_Dot.PPM")?;
    file.write_all(canvas.create_file_string().as_bytes())?;
    Ok(())
}