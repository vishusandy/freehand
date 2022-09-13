mod aa_oct;

pub(crate) mod aa_quad {
    use crate::Pt;
    use log::{debug, trace};

    #[derive(Clone, Debug)]
    pub struct AAArc {
        x: f64,
        y: f64,
        r: f64,
        r2: f64,
        // stop: f64,
        quad: u8, // current quadrant
        end: u8,  // end quadrant
        c: Pt<f64>,
    }
    impl AAArc {
        pub fn start(r: i32, c: Pt<i32>) -> Self {
            let r = r as f64;
            Self {
                x: 0.0,
                y: r,
                r,
                r2: r * r,
                // stop: (r / std::f64::consts::SQRT_2).round(),
                quad: 1,
                end: 4,
                c: c.f64(),
            }
        }

        fn next_quad(&mut self) -> bool {
            if self.y <= 0.0 {
                self.x = 0.0;
                self.y = self.r;
                self.quad = self.quad % 4 + 1;
                true
            } else {
                false
            }
        }

        /// Calculate the slow coordinate from the fast coordinate
        fn calc_slow(&self, fast: f64) -> f64 {
            (self.r2 - fast * fast).sqrt()
        }

        /// Returns the two slow coordinates to antialias and the distance between a and the actual arc (to be used for antialiasing)
        fn calc_fract(slow: f64) -> (f64, f64, f64) {
            let o = slow.fract();
            let a = slow.floor().abs();
            let b = a + 1.0;
            (a, b, o)
        }

        fn step(&mut self) -> Option<(Pt<u32>, Pt<u32>, f64)> {
            let (x, y) = (self.x, self.y);
            let rst;
            if self.x <= self.y {
                let (a, b, o) = Self::calc_fract(self.y);
                debug!("Oct7 x={} ya={} yb={} o={:.2}", self.x, a, b, o);
                rst = (
                    Pt::new(x, a).quad_to_iter(self.quad, self.c).u32(),
                    Pt::new(x, b).quad_to_iter(self.quad, self.c).u32(),
                    o,
                );
                self.x += 1.0;
                self.y = self.calc_slow(self.x);
            } else {
                let (a, b, o) = Self::calc_fract(self.x);
                debug!("Oct8 y={} xa={} xb={} o={:.2}", self.y, a, b, o);
                rst = (
                    Pt::new(a, y).quad_to_iter(self.quad, self.c).u32(),
                    Pt::new(b, y).quad_to_iter(self.quad, self.c).u32(),
                    o,
                );
                self.y -= 1.0;
                self.x = self.calc_slow(self.y);
            }
            trace!("    {:?}", rst);
            Some(rst)
        }
        fn end(&self) -> bool {
            self.quad == self.end && self.y <= 0.0
        }
    }

    impl Iterator for AAArc {
        type Item = (Pt<u32>, Pt<u32>, f64);
        fn next(&mut self) -> Option<Self::Item> {
            if self.end() {
                return None;
            }
            if self.next_quad() {
                return self.next();
            }
            self.step()
        }
    }

    fn plot_aa(image: &mut image::RgbaImage, a: Pt<u32>, b: Pt<u32>, o: u8, c: image::Rgba<u8>) {
        use image::Pixel;
        let c1 = image::Rgba([c[0], c[1], c[2], 255 - o]);
        let c2 = image::Rgba([c[0], c[1], c[2], o]);
        image.get_pixel_mut(a.x(), a.y()).blend(&c1);
        image.get_pixel_mut(b.x(), b.y()).blend(&c2);
    }

    fn opac(d: f64) -> u8 {
        use std::ops::Rem;
        (d * 255.0).round().rem(256.0) as u8
    }

    pub fn draw(image: &mut image::RgbaImage, iter: AAArc, color: image::Rgba<u8>) {
        for (a, b, d) in iter {
            let o = opac(d);
            plot_aa(image, a, b, o, color);
            trace!("    plot: o={} a={:?} b={:?}", o, a, b);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn aa_iter() -> Result<(), image::ImageError> {
            crate::logger(log::LevelFilter::Debug);
            let mut image = crate::guidelines();
            let iter = AAArc::start(crate::RADIUS, crate::CENTER.into());
            let color = image::Rgba([255, 0, 0, 255]);
            draw(&mut image, iter, color);
            image.save("images/aa_iter.png")
        }
    }
}
