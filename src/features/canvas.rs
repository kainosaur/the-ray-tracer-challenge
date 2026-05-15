use crate::features::tuple::Color;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color<f32>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0., 0., 0.); width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color<f32>) {
        if x > self.width {
            panic!("width out of bounds.");
        }
        let index = self.index(x, y);
        self.pixels[index] = color;
    }

    pub fn overwrite_all_pixels(&mut self, color: Color<f32>) {
        for i in 0..(self.width * self.height) {
            self.pixels[i] = color;
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color<f32> {
        self.pixels[self.index(x, y)]
    }

    pub fn create_file_string(&self) -> String {
        let mut s = String::new();
        let x_str = self.width.to_string();
        let y_str = self.height.to_string();
        // Header of PPM file.
        s.push_str("P3\n");
        s.push_str(&x_str);
        s.push_str(" ");
        s.push_str(&y_str);
        s.push_str("\n255\n");
        // Color data
        for i in 0..self.height {
            let mut line_len = 0;
            for j in 0..self.width {
                for component in self.pixels[self.index(j, i)].ppm_components() {
                    let component_len = component.len();

                    if line_len == 0 {
                        s.push_str(&component);
                        line_len = component_len;
                    } else if line_len + 1 + component_len > 70 {
                        s.push_str("\n");
                        s.push_str(&component);
                        line_len = component_len;
                    } else {
                        s.push_str(" ");
                        s.push_str(&component);
                        line_len += 1 + component_len;
                    }
                }
            }

            if i < self.height - 1 {
                s.push_str("\n");
            }
        }
        s
    }
}
