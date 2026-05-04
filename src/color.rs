use std::fmt::Display;

use dioxus::logger::tracing;

use palette::FromColor;
use palette::Hsl;
use palette::IntoColor;
use palette::Lab;
use palette::Srgb;
use palette::color_difference::Ciede2000;

#[derive(Clone, Copy, Debug, Default)]
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
            inner: Hsl::from_color(
                hex.parse::<Srgb<u8>>()
                    .expect("failed to parse hex color code to Srgb<u8>")
                    .into_format::<f32>(),
            ),
        }
    }

    // Construct a new `Color` by wrapping a Hsl color.
    pub const fn _from_hsl(hsl: Hsl) -> Self {
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

    // Convert this color into the closest possible one in the APB palette.
    pub fn to_apb(&self) -> ApbColor {
        let mut count = 0;

        let mut lowest_delta = f32::MAX;
        let mut lowest_color = ApbColor::default();

        let time_before: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

        for i in 0..32 {
            for j in 0..8 {
                for k in 0..16 {
                    let color = ApbColor {
                        hue: i,
                        saturation: j,
                        lightness: k,
                    };

                    let delta = self.difference(&Self::from_apb(color));

                    if delta < lowest_delta {
                        lowest_delta = delta;
                        lowest_color = color;
                    }

                    count += 1;
                }
            }
        }

        let time_elapsed = chrono::Utc::now() - time_before;
        let time_elapsed = time_elapsed.num_milliseconds();

        tracing::info!(
            "Computed {count} colors in {time_elapsed} ms, closest match is {lowest_delta:.2} DE2000"
        );

        lowest_color
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
