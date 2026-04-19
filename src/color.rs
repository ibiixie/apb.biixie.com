use std::fmt::Display;

use palette::FromColor;
use palette::Hsl;
use palette::IntoColor;
use palette::Lab;
use palette::Srgb;
use palette::color_difference::Ciede2000;

#[derive(Clone, Copy, Debug)]
pub struct ApbColor {
    pub hue: u8,
    pub saturation: u8,
    pub lightness: u8,
}

// Wraps `palette::Hsl` color with some useful additions.
#[derive(Clone, Default)]
pub struct ColorWrapper {
    inner: Hsl,
}

impl Display for ColorWrapper {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.to_hex())
    }
}

impl PartialEq for ColorWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl ColorWrapper {
    // Construct a new `Color` from a hex color code.
    pub fn from_hex(hex: &str) -> Self {
        Self {
            inner: Hsl::from_color(hex.parse::<Srgb<u8>>().unwrap().into_format::<f32>()),
        }
    }

    // Construct a new `Color` by wrapping a Hsl color.
    pub fn from_hsl(hsl: Hsl) -> Self {
        Self { inner: hsl }
    }

    pub fn from_apb(apb_color: ApbColor) -> Self {
        let quantize =
            |value: u8, range: f32, steps: u8| f32::from(value) * (range / f32::from(steps));

        Self {
            inner: Hsl {
                hue: quantize(apb_color.hue, 360.0, 31).into(),
                saturation: quantize(apb_color.saturation, 1.0, 7),
                lightness: quantize(apb_color.lightness, 1.0, 15),
                ..Hsl::default()
            },
        }
    }

    // Convert this color into a color that is compatible with the APB color palette.
    pub fn to_apb(&self) -> ApbColor {
        let quantize = |value: f32, range: f32, steps: u8| -> u8 {
            (value / (range / f32::from(steps))).round() as u8
        };

        ApbColor {
            hue: quantize(self.inner.hue.into_positive_degrees(), 360.0, 31),
            saturation: quantize(self.inner.saturation * 100.0, 100.0, 7),
            lightness: quantize(self.inner.lightness * 100.0, 100.0, 15),
        }
    }

    // Convert this color into a hex color code.
    pub fn to_hex(&self) -> String {
        let rgb: Srgb = self.inner.into_color();
        let rgb_u8 = rgb.into_format::<u8>();
        format!("#{:02X}{:02X}{:02X}", rgb_u8.red, rgb_u8.green, rgb_u8.blue)
    }

    // Get a reference to the inner Hsl struct.
    pub const fn inner(&self) -> &Hsl {
        &self.inner
    }

    // Calculate the difference between self and another color in CIE ∆E2000.
    pub fn difference(&self, other: &Self) -> f32 {
        let lab: Lab = self.inner.into_color();
        let other_lab: Lab = other.inner.into_color();

        lab.difference(other_lab)
    }
}
