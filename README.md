# Freehand

Freehand is a small Rust library with minimal dependencies that provides extra drawing utilities that utilize the [`image`](https://docs.rs/image/latest/image/) crate.

## Why?

- Only depends on the [`image`](https://docs.rs/image/latest/image/) crate.
- Provides some additional drawing functions not found in the [`imageproc`](https://docs.rs/imageproc/latest/imageproc/) (which also has many dependencies).

## Drawing

It provides the following for drawing:
- circular arcs
- partial annulii (basically a slice of a filled donut)
- antialiased circular arcs
- straight vertical, horizontal, and diagonal lines, with variants for dashed lines and alpha blended lines
- filled rectangles

## Unsafe

A few unsafe functions are provided to improve performance by removing redundant bounds checks and eliminating unnecessary branches.
This is useful when you have already checked the bounds or know the coordinates must be within the bounds.

## Limitations

Currently antialiasing and alpha blending are only supported when using an [`RgbaImage`](https://docs.rs/image/latest/image/type.RgbaImage.html) image.

## Todo

- Add more shapes
- Add elliptical arcs



