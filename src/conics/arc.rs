mod bounds;
mod edge;
mod pos;

use crate::{angle, translate, Pt};
use bounds::Bounds;
use edge::Edge;
use pos::Pos;

/// Draws a circular arc from a given start angle to an end angle.
///
/// A floating-point angle will represent an angle in radians.  Integer types
/// will represent an angle in degrees.
///
/// # Examples
///
/// Draws an arc that goes across the top half of the image (0° to 180°):
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::arc;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
/// arc(&mut image, start, end, radius, center, color);
/// ```
/// Integer numbers for angles are treated as degrees while floating-point numbers
/// are treated as radians.
///
/// This will draw the same image as above using radians (PI = 180°):
///
/// ```
/// # use image::{RgbaImage, Rgba};
/// # use freehand::conics::arc;
/// # let bg = Rgba([255, 255, 255, 255]); // white
/// # let color = Rgba([255, 0, 0, 255]);
/// # let mut image = RgbaImage::from_pixel(400, 400, bg);
/// # let radius = 190;
/// # let center = (200, 200);
/// let start = 0.0;
/// let end = std::f64::consts::PI;
/// arc(&mut image, start, end, radius, center, color);
/// ```
///
/// See also: [`Draw::arc`](crate::Draw::arc)
///
pub fn arc<A, C, I, T>(
    image: &mut I,
    start_angle: A,
    end_angle: A,
    radius: T,
    center: C,
    color: I::Pixel,
) where
    A: crate::Angle,
    C: crate::pt::Point<T>,
    I: image::GenericImage,
    T: Into<i32> + Copy,
{
    Arc::new(start_angle, end_angle, radius, center).draw(image, color);
}

/// A structure for iterating over points in a circular arc.
///
/// Does not implement the `Iterator` trait because points for even octants would
/// be returned in reverse order.
///
/// ```
/// use image::{RgbaImage, Rgba};
/// use freehand::conics::Arc;
///
/// let bg = Rgba([255, 255, 255, 255]); // white
/// let color = Rgba([255, 0, 0, 255]);
/// let mut image = RgbaImage::from_pixel(400, 400, bg);
///
/// /// An arc that goes across the top half of the image (0° to 180°)
/// let radius = 190;
/// let center = (200, 200);
/// let start = 0; // 0°
/// let end = 180; // 180°
///
/// /// Create the struct
/// let arc = Arc::new(start, end, radius, center);
///
/// /// Draw the struct
/// arc.draw(&mut image, color);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Arc {
    /// Current iteration position.
    pos: Pos,
    /// Angle and octant of the start edge
    start: Edge,
    /// Angle and octant of the end edge
    end: Edge,
    /// Center of the circular arc
    c: Pt<i32>,
    /// Radius of the arc
    r: i32,
    /// Used to determine when to iterate over all octants and back to the original octant.
    /// If `revisit` is true iteration will not immediately end when the octant is finished.
    /// This is set to true for the first octant when `start.oct == end.oct` and `start.angle > end.angle`
    revisit: bool,
}

impl Arc {
    /// Creates a new [`Arc`].
    ///
    /// Floating-point angles will represent an angle in radians.  Integer types
    /// will represent an angle in degrees.
    ///
    /// Negative angles are supported as well as angles larger than 360° (or
    /// larger than`2*PI` for radians).  Angles will be normalized into a range
    /// of 0..PI*2.
    ///
    /// # Panic
    ///
    /// Panics if radius is less than or equal to 0
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::Arc;
    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let arc = Arc::new(0, 180, 190, (200, 200));
    /// ```
    pub fn new<A, T, C>(start_angle: A, end_angle: A, radius: T, center: C) -> Self
    where
        A: crate::Angle,
        T: Into<i32> + Copy,
        C: crate::pt::Point<T>,
    {
        let start = angle::normalize(start_angle.radians());
        let end = angle::normalize(end_angle.radians() - crate::TINY);

        let mut arc = Self::blank(start, end, radius, center);
        let bounds = Bounds::start_bounds(&arc.start, &arc.end, arc.revisit);

        arc.pos = Pos::new(arc.start.oct, bounds, arc.r, arc.c);
        arc
    }

    fn blank<T, C>(start_angle: f64, end_angle: f64, r: T, c: C) -> Self
    where
        T: Into<i32> + Copy,
        C: crate::pt::Point<T>,
    {
        let c = Pt::new(c.x().into(), c.y().into());
        let r = r.into();

        assert!(r > 0, "Radius must be larger than 0.  radius={r}");

        let start_oct = crate::angle::angle_to_octant(start_angle);
        let end_oct = crate::angle::angle_to_octant(end_angle);

        Self {
            pos: Pos::start(start_oct, r),
            start: Edge::new(start_angle, start_oct),
            end: Edge::new(end_angle, end_oct),
            c,
            r,
            revisit: start_oct == end_oct && start_angle > end_angle,
        }
    }

    /// Create an iterator over a single circular octant
    ///
    /// # Panics
    ///
    /// Panics if radius is less than or equal to 0
    ///
    pub fn octant<T, C>(oct: u8, r: T, c: C) -> Self
    where
        C: crate::pt::Point<T>,
        T: Into<i32> + Copy,
    {
        let c = Pt::new(c.x().into(), c.y().into());
        let r = r.into();

        assert!(r > 0, "Radius be must larger than 0");

        assert!(
            (1..=8).contains(&oct),
            "Invalid octant. Valid octants are 1 through 8"
        );

        let pos = Pos::start(oct, r);

        let start = Edge::new(angle::octant_start_angle(oct), oct);
        let end = Edge::new(angle::octant_end_angle(oct), oct);

        Self {
            pos,
            start,
            end,
            c,
            r,
            revisit: false,
        }
    }

    pub(super) fn restart(&mut self) {
        let oct = self.pos.oct % 8 + 1;
        let bounds = Bounds::bounds_from_edges(oct, &self.start, &self.end, self.revisit);
        self.pos = Pos::new(oct, bounds, self.r, self.c);
        self.revisit = false;
    }

    pub(super) fn end(&self) -> bool {
        self.pos.oct == self.end.oct && !self.revisit
    }

    /// Draw the specified arc by iterating over its points.
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # use freehand::conics::Arc;
    ///
    /// # let bg = Rgba([255, 255, 255, 255]); // white
    /// # let mut image = RgbaImage::from_pixel(400, 400, bg);
    ///
    /// let arc = Arc::new(0, 180, 190, (200, 200));
    /// arc.draw(&mut image, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    pub fn draw<I>(mut self, image: &mut I, color: I::Pixel)
    where
        I: image::GenericImage,
    {
        loop {
            if self.pos.stop() {
                if self.end() {
                    break;
                }
                self.restart();
                continue;
            }

            let pt: Result<Pt<u32>, &'static str> = self.pt().try_into();
            if let Ok(pt) = pt {
                if pt.x() < image.width() && pt.y() < image.height() {
                    image.put_pixel(pt.x(), pt.y(), color);
                }
            }
            self.pos.inc();
        }
    }

    /// Helper function to translate the current coordinates into a specified octant
    pub(super) fn coords_oct(&self, oct: u8) -> Pt<i32> {
        let pt = Pt::new(self.pos.x, self.pos.y);
        translate::iter_to_real(pt.x(), pt.y(), oct, self.c)
    }

    pub(super) fn pt(&self) -> Pt<i32> {
        let pt = Pt::new(self.pos.x, self.pos.y);
        translate::iter_to_real(pt.x(), pt.y(), self.pos.oct, self.c)
    }

    /// Helper function for other modules
    pub(super) fn stop(&self) -> bool {
        self.pos.stop()
    }

    /// Helper function for other modules
    pub(super) fn inc(&mut self) {
        self.pos.inc();
    }

    /// Returns the center coordinates
    #[must_use]
    pub fn center(&self) -> Pt<i32> {
        self.c
    }

    /// Returns the radius
    #[must_use]
    pub fn radius(&self) -> i32 {
        self.r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RADS;

    #[test]
    fn arc_draw() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);

        let r = 190;
        let c = (200, 200);
        let start = RADS * 1.8;
        let end = RADS * 0.5;

        let mut image = crate::circle_guides(r);
        let arc = Arc::new(start, end, r, c);

        arc.draw(&mut image, image::Rgba([255, 0, 0, 255]));

        image.save("images/arc.png")
    }
}
