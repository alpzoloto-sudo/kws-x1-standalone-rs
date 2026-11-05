//! Experimental meter-temperature estimate from the stock firmware's PA5
//! thermistor divider.
//!
//! The stock firmware converts an ADC2 count `x` with a beta model
//! (`B = 3380 K`, `T0 = 298.15 K`) and the divider ratio `(4095 - x) / x`.
//! The physical sensor, its divider and the actual beta value are not
//! verified on this hardware. Like the stock equation, the raw count assumes
//! the divider supply tracks the ADC's full-scale voltage. A floating PA5 can
//! produce a plausible-looking value, and this code cannot detect an absent sensor.
//! The result is a rough estimate of the meter's own temperature, not the
//! cable, charger or ambient temperature, and it is neither calibrated nor
//! suitable for safety decisions.
//!
//! The conversion is integer-only: a Q8 base-2 logarithm with a small
//! interpolated table, then the beta equation in microreciprocal kelvins.
//! That keeps the estimate within about 1 C of the ideal beta curve over the
//! accepted range, without pulling floating-point math into the firmware.

/// 12-bit ADC full scale, as the stock equation uses.
const FULL_SCALE: i32 = 4095;
/// Beta in kelvins, as the stock firmware uses.
const BETA_K: i32 = 3380;
/// `1 / T0` in microreciprocal kelvins for `T0 = 298.15 K` (25 C).
const INV_T0_UK: i32 = 3354;
/// `ln(2) * 1e9 / (BETA_K * 256)`, rounded: thousandths of a
/// microreciprocal kelvin per Q8 log2 unit.
const LN2_UK_PER_LOG2_Q8: i32 = (693_147 * 1000 + BETA_K * 256 / 2) / (BETA_K * 256);
/// Plausible range of the estimate, in degrees Celsius. Outside it the
/// reading is treated as invalid rather than reported as a plausible value.
const MIN_C: i16 = -40;
const MAX_C: i16 = 125;
/// Q8 `log2(1 + i/16)`, rounded, for `i = 0..=16`. The last entry is 255
/// rather than 256 so the table fits `u8`; the difference is far below the
/// estimate's resolution.
const LOG2_FRAC: [u8; 17] = [
    0, 22, 44, 63, 82, 100, 118, 134, 150, 165, 179, 193, 207, 220, 232, 244, 255,
];

/// Estimate the meter temperature in degrees Celsius from one PA5 ADC2
/// count. `None` for a rail/saturated count, an invalid divider ratio, or a
/// result outside [`MIN_C`]..=[`MAX_C`].
pub fn estimate_c(count: u16) -> Option<i16> {
    let count = i32::from(count);
    if count <= 0 || count >= FULL_SCALE {
        return None;
    }
    let log2_q8 = log2_q8(FULL_SCALE - count) - log2_q8(count);
    let inv_t_uk = INV_T0_UK + log2_q8 * LN2_UK_PER_LOG2_Q8 / 1000;
    if inv_t_uk <= 0 {
        return None;
    }
    let kelvin = (1_000_000 + inv_t_uk / 2) / inv_t_uk;
    let celsius = i16::try_from(kelvin - 273).ok()?;
    (MIN_C..=MAX_C).contains(&celsius).then_some(celsius)
}

/// `log2(n)` in Q8 for `n > 0`: the integer exponent plus a linearly
/// interpolated 16-segment mantissa table.
fn log2_q8(n: i32) -> i32 {
    let n = n as u32;
    let exponent = n.ilog2() as i32;
    let mantissa = (n << (31 - exponent)) >> 23;
    let offset = mantissa as i32 - 256;
    let index = (offset >> 4) as usize;
    let fraction = offset & 0xf;
    let base = i32::from(LOG2_FRAC[index]);
    let step = i32::from(LOG2_FRAC[index + 1]) - base;
    (exponent << 8) + base + step * fraction / 16
}
