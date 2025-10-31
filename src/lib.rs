use image::{DynamicImage, GenericImageView};

/// Converts RGB into an ANSI color code.
fn ansi(r: u8, g: u8, b: u8, bg: bool) -> String {
    return format!("\x1b[{};2;{};{};{}m", if bg { "48" } else { "38" }, r, g, b);
}

pub trait Trim {
    /// Remove whitespace from an image, useful to make sure the image fits.
    fn trim(self) -> Self;
}

impl Trim for DynamicImage {
    fn trim(self) -> Self {
        let dimensions = self.dimensions();
        let mut min = dimensions;
        let mut max = (0, 0);

        for (x, y, pixel) in self.pixels() {
            if pixel[3] != 0 {
                min.0 = min.0.min(x);
                max.0 = max.0.max(x);
                min.1 = min.1.min(y);
                max.1 = max.1.max(y);
            }
        }

        if max.0 < min.0 || max.1 < min.1 {
            return self;
        }

        self.crop_imm(min.0, min.1, max.0 - min.0 + 1, max.1 - min.1 + 1)
    }
}

/// Does the actual conversion to text.
/// This uses the `▀` and `▄` characters and requires fullcolor.
/// Max image size on standard sized terminal is 80x96.
pub fn render(img: &DynamicImage) -> String {
    let mut out = String::new();
    let (width, height) = img.dimensions();

    // Get every other row
    for y in (0..height - 1).step_by(2) {
        for x in 0..width {
            // The first pixel.
            let pixel = img.get_pixel(x, y);

            // The next pixel under the first. It's done like this because pixels are printed
            // twice at a time; one on top, and one on the bottom. This is so we can make the sprite
            // as small as possible, leaving more room.
            let next = img.get_pixel(x, y + 1);

            // If it's the start of a row, and it isn't the first, print a newline.
            if x == 0 && y != 0 {
                out.push('\n')
            }

            // If both pixels are transparent, just print a space.
            if pixel[3] <= 0 && next[3] <= 0 {
                out.push(' ');
                continue;
            }

            if pixel[3] != 0 && next[3] != 0 {
                out.push_str(&ansi(pixel[0], pixel[1], pixel[2], true));
                out.push_str(&ansi(next[0], next[1], next[2], false));
            } else if pixel[3] == 0 {
                out.push_str(&ansi(next[0], next[1], next[2], false));
            } else if next[3] == 0 {
                out.push_str(&ansi(pixel[0], pixel[1], pixel[2], false));
            }

            if pixel[3] != 0 && next[3] == 0 {
                out.push_str("▀\x1b[0m");
            } else {
                out.push_str("▄\x1b[0m");
            }
        }
    }

    // If the height is odd, we have to also print the very bottom row in a special way.
    if height % 2 != 0 {
        out.push('\n');

        for x in 0..width {
            let pixel = img.get_pixel(x, height - 1);

            if pixel[3] <= 0 {
                out.push_str(" ");
                continue;
            }

            out.push_str(&ansi(pixel[0], pixel[1], pixel[2], false));
            out.push_str("▀\x1b[0m");
        }
    }

    return out;
}
