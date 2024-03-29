//! Allows drawing functions to be called using method chaining.
//!
//! This is a simple wrapper around a mutable image reference for convenience.
//!

// Draw's methods mutate the image and return self for convenience,
// so ignoring a return value is just fine
#![allow(clippy::return_self_not_must_use)]
// same reasoning as above
#![allow(clippy::must_use_candidate)]

use crate::conics;
use crate::lines;
use crate::ops;
use crate::shapes;
use crate::{Angle, Point, Pt};
use image::{GenericImage, Rgba, RgbaImage};

/// Allows drawing functions to be called using method chaining.
///
/// This is a simple wrapper around a mutable image reference.
///
/// # Example
///
/// ```
/// # use image::{Rgba, RgbaImage};
/// let mut image = RgbaImage::new(400, 400);
/// let color = Rgba([255, 0, 0, 255]);
///
/// let draw = freehand::new(&mut image);
/// // Draw a rectangle using lines
/// draw.line((10, 10), (50, 10), color)
///     .line((50, 10), (50, 50), color)
///     .line((50, 50), (10, 50), color)
///     .line((10, 50), (10, 10), color);
/// ```
pub struct Draw<'i, I>
where
    I: image::GenericImage,
{
    image: &'i mut I,
}

/// Methods for working with [`image::GenericImage`]s
///
/// [`image::GenericImage`]: https://docs.rs/image/latest/image/trait.GenericImage.html
impl<'i, I> Draw<'i, I>
where
    I: GenericImage,
{
    /// Creates a new wrapper around a mutable image refernce.
    ///
    /// This allows drawing functions to be called using method chaining.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{Rgba, RgbaImage};
    /// let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::Draw::new(&mut image);
    /// ```
    pub fn new(image: &'i mut I) -> Self {
        Self { image }
    }

    /// Draws a straight line.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a line between the two points
    /// draw.line((10, 10), (120, 180), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::line`]
    ///
    pub fn line<P, T>(self, a: P, b: P, color: I::Pixel) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        lines::line(self.image, a, b, color);
        self
    }

    /// Draws a dashed line between two points.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a 3px dashed line between the two points
    /// draw.dashed_line((10, 10), (120, 180), 3, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::dashed_line`]
    ///
    pub fn dashed_line<P, T>(self, a: P, b: P, dash_width: u16, color: I::Pixel) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        lines::dashed_line(self.image, a, b, dash_width, color);
        self
    }

    /// Draws a line from each point to the next.
    ///
    /// Does not connect the start and end points.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a line between each of the points
    /// let points = [(10, 10), (120, 180)];
    /// draw.path(points, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::path`]
    ///
    pub fn path<P, It>(self, points: It, color: I::Pixel) -> Self
    where
        P: Point<i32>,
        It: IntoIterator<Item = P>,
    {
        lines::path(self.image, points, color);
        self
    }

    /// Draws a rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// draw.rectangle((10, 10), 50, 50, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`shapes::rectangle`]
    ///
    pub fn rectangle<P>(self, pt: P, height: u32, width: u32, color: I::Pixel) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle(self.image, pt, height, width, color);
        self
    }

    /// Draws a filled rectangle
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// draw.rectangle_filled((10, 10), 50, 50, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`shapes::rectangle_filled`]
    ///
    pub fn rectangle_filled<P>(self, pt: P, height: u32, width: u32, color: I::Pixel) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_filled(self.image, pt, height, width, color);
        self
    }

    /// Draws a circular arc.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red arc from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.arc(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::arc`]
    ///
    pub fn arc<A, C, T>(
        self,
        start_angle: A,
        end_angle: A,
        radius: T,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<T>,
        T: Into<i32> + Copy,
    {
        conics::arc(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a circle.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red circle with a radius of 180 pixels from the image center.
    /// draw.circle(180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::circle`]
    ///
    pub fn circle<C, T>(self, radius: T, center: C, color: I::Pixel) -> Self
    where
        C: Point<T>,
        T: Into<i32> + Copy,
    {
        conics::circle(self.image, radius, center, color);
        self
    }

    /// Draws a filled pie slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a pie slice from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.pie_slice_filled(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::pie_slice_filled`]
    ///
    pub fn pie_slice_filled<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        radius: i32,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
        I: GenericImage,
    {
        conics::pie_slice_filled(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a thick arc.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws an arc, with a thickness of 3, from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.thick_arc(0, 55, 180, 3, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::thick_arc`]
    ///
    pub fn thick_arc<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        radius: i32,
        thickness: i16,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
    {
        conics::thick_arc(
            self.image,
            start_angle,
            end_angle,
            radius,
            thickness,
            center,
            color,
        );
        self
    }

    /// Draws a thick circle.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a circle with a thickness of 3 and a radius of 180 pixels from the image center.
    /// draw.thick_circle(180, 3, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::thick_circle`]
    ///
    pub fn thick_circle<C>(self, radius: i32, thickness: i16, center: C, color: I::Pixel) -> Self
    where
        C: Point<i32>,
    {
        conics::thick_circle(self.image, radius, thickness, center, color);
        self
    }

    /// Draws an annulus (a filled donut)
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws an annulus from 0° to 55°, with an inner radius of 120 and outer radius of 180 pixels from the image center.
    /// draw.annulus(0, 55, 120, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::annulus`]
    ///
    pub fn annulus<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        inner_radius: i32,
        outer_radius: i32,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
    {
        conics::annulus(
            self.image,
            start_angle,
            end_angle,
            inner_radius,
            outer_radius,
            center,
            color,
        );
        self
    }
}

/// Methods for working with [`image::RgbaImage`]s.
///
/// [`image::RgbaImage`]: https://docs.rs/image/latest/image/type.RgbaImage.html
impl<'i> Draw<'i, RgbaImage> {
    /// Draws an antialiased arc.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // draws an anti-aliased arc from 0° to 55° with a radius of 180 pixels from the image center.
    /// draw.antialiased_arc(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`conics::antialiased_arc`]
    ///
    pub fn antialiased_arc<A, C, T>(
        self,
        start_angle: A,
        end_angle: A,
        radius: T,
        center: C,
        color: Rgba<u8>,
    ) -> Self
    where
        A: Angle,
        C: Point<T>,
        T: Into<f64> + Copy,
    {
        conics::antialiased_arc(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a dashed line with a specified opacity.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red line with a 3px dash and 50% opacity.
    /// draw.dashed_line_alpha((0, 10), (200, 200), 5u8, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::dashed_line_alpha`]
    pub fn dashed_line_alpha<P, W>(
        self,
        a: P,
        b: P,
        dash_width: W,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<i32>,
        W: Into<u16>,
    {
        lines::dashed_line_alpha(self.image, a, b, dash_width, opacity, color);
        self
    }

    /// Draws a line with a specified opacity.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red line with 50% opacity.
    /// draw.line_alpha((0, 10), (200, 200), 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::line_alpha`]
    ///
    pub fn line_alpha<P>(self, a: P, b: P, opacity: f32, color: Rgba<u8>) -> Self
    where
        P: Point<i32>,
    {
        lines::line_alpha(self.image, a, b, opacity, color);
        self
    }

    /// Draws a thick anti-aliased line.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red anti-aliased line with a width of 1.5
    /// draw.antialiased_line((0, 10), (200, 200), 1.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`lines::antialiased_line`]
    ///
    pub fn antialiased_line<P, T>(self, a: P, b: P, width: f32, color: Rgba<u8>) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        lines::antialiased_line(self.image, a, b, width, color);
        self
    }

    /// Draws a rectangle with the specified opacity.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red rectangle with 50% opacity.
    /// draw.rectangle_alpha((0, 10), 50, 50, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`shapes::rectangle_alpha`]
    ///
    pub fn rectangle_alpha<P>(
        self,
        pt: P,
        height: u32,
        width: u32,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_alpha(self.image, pt, height, width, opacity, color);
        self
    }

    /// Draws a filled rectangle with the specified opacity.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a filled red rectangle with 50% opacity.
    /// draw.rectangle_filled_alpha((0, 10), 50, 50, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`shapes::rectangle_filled_alpha`]
    ///
    pub fn rectangle_filled_alpha<P>(
        self,
        pt: P,
        height: u32,
        width: u32,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_filled_alpha(self.image, pt, height, width, opacity, color);
        self
    }

    /// Blends a color into an image.
    ///
    /// The resulting color's alpha channel will ignore the specified color's alpha
    /// value and use `opacity` to blend the colors together.  The specified
    /// color's alpha value will only be used for the final alpha channel value.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Blends a red pixel into the image at (0, 10) with 50% opacity
    /// draw.blend_at(0, 10, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`ops::blend_at`]
    ///
    pub fn blend_at(self, x: u32, y: u32, opacity: f32, color: Rgba<u8>) -> Self {
        ops::blend_at(self.image, x, y, opacity, color);
        self
    }

    /// Blend a specified color into an existing image coordinate.  This ignores `color`'s
    /// alpha value and instead uses `opacity` which is a floating point number from 0.0 to 1.0.
    ///
    /// The resulting color's alpha channel will ignore the specified color's alpha
    /// value and use `opacity` to blend the colors together.  The specified
    /// color's alpha value will only be used for the final alpha channel value.
    ///
    /// A few safety checks are skipped here for performance.
    ///
    /// # Safety
    /// The x and y coordinates must be less than the image width and height, respectively.
    ///
    /// Also, `opacity` should be in the range `(0..=1.0)`.
    ///
    /// # Example
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Blends a red pixel into the image at (0, 10) with 50% opacity
    /// draw.blend_at(0, 10, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    ///
    /// See [`ops::blend_at_unchecked`]
    ///
    pub unsafe fn blend_at_unchecked(self, x: u32, y: u32, opacity: f32, color: Rgba<u8>) -> Self {
        ops::blend_at_unchecked(self.image, x, y, opacity, color);
        self
    }
}

/// Creates a new [`Draw`] struct for a mutable image.
///
/// This allows drawing functions to be called using method chaining.
///
/// # Example
///
/// ```
/// # use image::{Rgba, RgbaImage};
/// let mut image = RgbaImage::new(400, 400);
///
/// let draw = freehand::new(&mut image);
/// ```
pub fn new<I>(image: &mut I) -> Draw<I>
where
    I: image::GenericImage,
{
    Draw { image }
}
