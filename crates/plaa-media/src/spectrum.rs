//! How loud sound is across its pitches, for the visualizer: a Fourier
//! transform of the frames heard last, gathered into bands spaced evenly
//! by octave, as the ear hears pitch.

use std::f64::consts::TAU;

/// The frames each look takes: 85 ms at 48 kHz, enough to tell the bass's
/// pitches apart.
pub const FRAMES: usize = 4096;

/// The lowest and highest pitches shown, in hertz.
const LOWEST: f64 = 40.0;
const HIGHEST: f64 = 16_000.0;

/// The quietest level shown, in decibels under a full-scale sine.
const FLOOR: f32 = -70.0;

/// Decibels added for each octave above 1 kHz and taken off below, as
/// music's highs are quieter than its lows.
const TILT: f32 = 3.0;

/// The transform's tables and room, kept from one look to the next.
pub struct Spectrum {
    /// A Hann window, so a pitch between bands does not spread over all.
    window: Vec<f32>,
    /// Each step around the circle the transform turns by, as its cosine
    /// and sine.
    turns: Vec<(f32, f32)>,
    real: Vec<f32>,
    imaginary: Vec<f32>,
}

impl Default for Spectrum {
    fn default() -> Self {
        Self::new()
    }
}

impl Spectrum {
    pub fn new() -> Self {
        let window = (0..FRAMES)
            .map(|index| (0.5 - 0.5 * (TAU * index as f64 / FRAMES as f64).cos()) as f32)
            .collect();
        Self {
            window,
            turns: turns(FRAMES),
            real: vec![0.0; FRAMES],
            imaginary: vec![0.0; FRAMES],
        }
    }

    /// The level of each of `count` bands, the lowest first, from 0 at the
    /// floor or quieter to 1 at full scale, of `frames` played at `rate`:
    /// the last FRAMES of them, the newest last.
    pub fn bands(&mut self, frames: &[f32], rate: u32, count: usize) -> Vec<f32> {
        let frames = &frames[frames.len().saturating_sub(FRAMES)..];
        let missing = FRAMES - frames.len();
        self.real[..missing].fill(0.0);
        for ((real, frame), weight) in self.real[missing..]
            .iter_mut()
            .zip(frames)
            .zip(&self.window[missing..])
        {
            *real = frame * weight;
        }
        self.imaginary.fill(0.0);
        transform(&mut self.real, &mut self.imaginary, &self.turns);
        // A full-scale sine comes to a quarter of the frames, half lost
        // to the window.
        let scale = 4.0 / FRAMES as f32;
        let magnitudes: Vec<f32> = self.real[..FRAMES / 2]
            .iter()
            .zip(&self.imaginary[..FRAMES / 2])
            .map(|(real, imaginary)| real.hypot(*imaginary) * scale)
            .collect();
        let rate = f64::from(rate.max(1));
        let width = rate / FRAMES as f64;
        let highest = HIGHEST.min(rate / 2.0);
        let ratio = (highest / LOWEST).powf(1.0 / count.max(1) as f64);
        (0..count)
            .map(|band| {
                let low = LOWEST * ratio.powi(band as i32);
                let high = low * ratio;
                let middle = (low * high).sqrt();
                let first = (low / width).ceil() as usize;
                let last = ((high / width).ceil() as usize).min(magnitudes.len());
                // A low band can fall between two of the transform's
                // pitches: then it is read between them.
                let magnitude = if first < last {
                    magnitudes[first..last].iter().copied().fold(0.0, f32::max)
                } else {
                    let at = middle / width;
                    let below = (at.floor() as usize).min(magnitudes.len() - 1);
                    let above = (below + 1).min(magnitudes.len() - 1);
                    let share = (at - at.floor()) as f32;
                    magnitudes[below] * (1.0 - share) + magnitudes[above] * share
                };
                let decibels =
                    20.0 * magnitude.max(1e-9).log10() + TILT * (middle / 1000.0).log2() as f32;
                ((decibels - FLOOR) / -FLOOR).clamp(0.0, 1.0)
            })
            .collect()
    }
}

/// The steps around the circle a transform of `size` turns by.
fn turns(size: usize) -> Vec<(f32, f32)> {
    (0..size / 2)
        .map(|step| {
            let angle = TAU * step as f64 / size as f64;
            (angle.cos() as f32, angle.sin() as f32)
        })
        .collect()
}

/// The discrete Fourier transform of `real` and `imaginary` in place, by
/// halving: their length a power of two, `turns` made for it.
fn transform(real: &mut [f32], imaginary: &mut [f32], turns: &[(f32, f32)]) {
    let size = real.len();
    let bits = size.trailing_zeros();
    for index in 0..size {
        let reversed = index.reverse_bits() >> (usize::BITS - bits);
        if reversed > index {
            real.swap(index, reversed);
            imaginary.swap(index, reversed);
        }
    }
    let mut span = 2;
    while span <= size {
        let half = span / 2;
        let stride = size / span;
        for start in (0..size).step_by(span) {
            for offset in 0..half {
                let (cos, sin) = turns[offset * stride];
                let (even, odd) = (start + offset, start + offset + half);
                let turned_real = real[odd] * cos + imaginary[odd] * sin;
                let turned_imaginary = imaginary[odd] * cos - real[odd] * sin;
                real[odd] = real[even] - turned_real;
                imaginary[odd] = imaginary[even] - turned_imaginary;
                real[even] += turned_real;
                imaginary[even] += turned_imaginary;
            }
        }
        span *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_transform_is_fouriers() {
        let size = 64;
        let input: Vec<f32> = (0..size)
            .map(|index| ((index * 7 + 3) % 11) as f32 - 5.0)
            .collect();
        let mut real = input.clone();
        let mut imaginary = vec![0.0; size];
        transform(&mut real, &mut imaginary, &turns(size));
        for pitch in 0..size {
            let (mut expected_real, mut expected_imaginary) = (0.0f64, 0.0f64);
            for (index, value) in input.iter().enumerate() {
                let angle = TAU * (pitch * index) as f64 / size as f64;
                expected_real += f64::from(*value) * angle.cos();
                expected_imaginary -= f64::from(*value) * angle.sin();
            }
            assert!((f64::from(real[pitch]) - expected_real).abs() < 1e-3);
            assert!((f64::from(imaginary[pitch]) - expected_imaginary).abs() < 1e-3);
        }
    }

    #[test]
    fn a_tone_shows_in_its_band() {
        let rate = 48_000;
        let count = 32;
        let tone: Vec<f32> = (0..FRAMES)
            .map(|index| 0.5 * (TAU * 1000.0 * index as f64 / f64::from(rate)).sin() as f32)
            .collect();
        let bands = Spectrum::new().bands(&tone, rate, count);
        let ratio = (HIGHEST / LOWEST).powf(1.0 / count as f64);
        let band = ((1000.0 / LOWEST).ln() / ratio.ln()) as usize;
        // Half of full scale is 6 dB under it.
        let expected = (-6.0 - FLOOR) / -FLOOR;
        assert!(
            (bands[band] - expected).abs() < 0.03,
            "{} for {expected}",
            bands[band]
        );
        // An octave away, a good deal quieter.
        let octave = (2.0f64.ln() / ratio.ln()).round() as usize;
        assert!(bands[band - octave] < expected - 0.3);
        assert!(bands[band + octave] < expected - 0.3);
    }

    #[test]
    fn silence_shows_nothing_and_short_sound_is_taken() {
        let mut spectrum = Spectrum::new();
        assert!(
            spectrum
                .bands(&[0.0; FRAMES], 44_100, 16)
                .iter()
                .all(|&level| level == 0.0)
        );
        // Fewer frames than a look takes, as when playback starts.
        let bands = spectrum.bands(&[0.25; 100], 44_100, 16);
        assert_eq!(bands.len(), 16);
    }
}
