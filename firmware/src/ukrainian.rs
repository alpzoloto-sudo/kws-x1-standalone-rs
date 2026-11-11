//! Compact 5x7 Ukrainian uppercase glyphs for the 240x135 meter display.
//! Kept separate from the manufacturer's fonts to avoid missing Cyrillic glyphs.
//! This is an initial localization: unlisted technical labels stay in English.
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

/// Translate stable, user-facing labels without modifying protocol strings.
pub fn translate(source: &str) -> Option<&'static str> {
    Some(match source {
        "SETTINGS" => "НАЛАШТУВАННЯ",
        "OVERVIEW" => "ОГЛЯД",
        "GRAPH" => "ГРАФІК",
        "RIPPLE" => "ПУЛЬСАЦІЇ",
        "LINES" => "ЛІНІЇ",
        "ENERGY" => "ЕНЕРГІЯ",
        "CHARGER" => "ЗАРЯДНИК",
        "CABLE" => "КАБЕЛЬ",
        "SYSTEM" => "СИСТЕМА",
        "TIME" => "ЧАС",
        "CHARGE" => "ЗАРЯД",
        "NOW" => "ЗАРАЗ",
        "NOW/MAX" => "ЗАРАЗ МАКС",
        "WAIT" => "ОЧІКУЙТЕ",
        "ACTIVE" => "АКТИВНО",
        "FAILED" => "ПОМИЛКА",
        "BACK" => "НАЗАД",
        "OUTPUT" => "ВИХІД",
        "MODE" => "РЕЖИМ",
        "STEPS" => "КРОКИ",
        "DETECTED" => "ЗНАЙДЕНО",
        "SUPPORTED" => "ПІДТРИМКА",
        "NOT SUPPORTED" => "НЕ ПІДТРИМУЄ",
        "NOT APPLIED" => "НЕ ЗАСТОСОВАНО",
        "MAX VOLTAGE" => "МАКС НАПРУГА",
        "MAX CURRENT" => "МАКС СТРУМ",
        "SET VOLTAGE" => "ЗАДАТИ НАПРУГУ",
        "CURRENT LIMIT" => "МЕЖА СТРУМУ",
        "VOLTAGE" => "НАПРУГА",
        "CURRENT" => "СТРУМ",
        "POWER" => "ПОТУЖНІСТЬ",
        "MAX" => "МАКС",
        "PROTOCOL" => "ПРОТОКОЛ",
        "BRIGHTNESS" => "ЯСКРАВІСТЬ",
        "ROTATION" => "ОБЕРТАННЯ",
        "ENERGY MEMORY" => "ПАМ'ЯТЬ",
        "TIMER" => "ТАЙМЕР",
        "CALIBRATION" => "КАЛІБРУВАННЯ",
        "AUTO" => "АВТО",
        "NORMAL" => "ЗВИЧАЙНО",
        "FLIPPED" => "ПОВЕРНУТО",
        _ => return None,
    })
}

// A compact lookup table avoids one compiled match arm per Unicode glyph.
// 32 consecutive Unicode Cyrillic codepoints, then four Ukrainian extensions,
// space and apostrophe; zero rows for unused Russian-only letters.
const GLYPHS: [[u8; 7]; 38] = [
    [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // А
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b11110], // Б
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110], // В
    [0b11111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000], // Г
    [0b00110, 0b01010, 0b01010, 0b01010, 0b01010, 0b11111, 0b10001], // Д
    [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111], // Е
    [0b10101, 0b10101, 0b10101, 0b01110, 0b10101, 0b10101, 0b10101], // Ж
    [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110], // З
    [0b10001, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b10001], // И
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b10001], // Й
    [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001], // К
    [0b00111, 0b01001, 0b01001, 0b01001, 0b01001, 0b10001, 0b10001], // Л
    [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001], // М
    [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // Н
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // О
    [0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001], // П
    [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000], // Р
    [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111], // С
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100], // Т
    [0b10001, 0b10001, 0b10001, 0b01111, 0b00001, 0b10001, 0b01110], // У
    [0b00100, 0b01110, 0b10101, 0b10101, 0b01110, 0b00100, 0b00100], // Ф
    [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001], // Х
    [0b10010, 0b10010, 0b10010, 0b10010, 0b10010, 0b11111, 0b00001], // Ц
    [0b10001, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b00001], // Ч
    [0b10101, 0b10101, 0b10101, 0b10101, 0b10101, 0b10101, 0b11111], // Ш
    [0b10101, 0b10101, 0b10101, 0b10101, 0b10101, 0b11111, 0b00001], // Щ
    [0, 0, 0, 0, 0, 0, 0], // Ъ
    [0, 0, 0, 0, 0, 0, 0], // Ы
    [0b10000, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b11110], // Ь
    [0, 0, 0, 0, 0, 0, 0], // Э
    [0b10010, 0b10101, 0b10101, 0b11101, 0b10101, 0b10101, 0b10010], // Ю
    [0b01111, 0b10001, 0b10001, 0b01111, 0b00101, 0b01001, 0b10001], // Я
    [0b00001, 0b11111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000], // Ґ
    [0b01111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b01111], // Є
    [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111], // І
    [0b01010, 0b00000, 0b11111, 0b00100, 0b00100, 0b00100, 0b11111], // Ї
    [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000], //  
    [0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000], // '
];

fn glyph(ch: char) -> Option<&'static [u8; 7]> {
    let index = match ch {
        'А'..='Я' => ch as usize - 'А' as usize,
        'Ґ' => 32,
        'Є' => 33,
        'І' => 34,
        'Ї' => 35,
        ' ' => 36,
        '\'' => 37,
        _ => return None,
    };
    Some(&GLYPHS[index])
}

/// Draw a translated label with clipping handled by DrawTarget.
/// Each character occupies six pixels horizontally; height is seven pixels.
/// The baseline convention matches the existing UI text helper.
pub fn draw<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    message: &str,
    x: i32,
    baseline: i32,
    alignment: u8, // 0 left, 1 center, 2 right
    ink: Rgb565,
) -> i32 {
    let width = message.chars().count() as i32 * 6;
    let start = match alignment {
        1 => x - width / 2,
        2 => x - width,
        _ => x,
    };
    for (index, ch) in message.chars().enumerate() {
        if let Some(rows) = glyph(ch) {
            for (row, bits) in rows.iter().enumerate() {
                for column in 0..5 {
                    if bits & (1 << (4 - column)) != 0 {
                        let point = Point::new(
                            start + index as i32 * 6 + column,
                            baseline - 7 + row as i32,
                        );
                        let _ = target.draw_iter(core::iter::once(Pixel(point, ink)));
                    }
                }
            }
        }
    }
    start + width
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_localized_label_has_glyphs() {
        let labels = [
            "SETTINGS",
            "OVERVIEW",
            "GRAPH",
            "RIPPLE",
            "LINES",
            "ENERGY",
            "CHARGER",
            "CABLE",
            "SYSTEM",
            "TIME",
            "CHARGE",
            "NOW",
            "NOW/MAX",
            "WAIT",
            "ACTIVE",
            "FAILED",
            "BACK",
            "OUTPUT",
            "MODE",
            "STEPS",
            "DETECTED",
            "SUPPORTED",
            "NOT SUPPORTED",
            "NOT APPLIED",
            "MAX VOLTAGE",
            "MAX CURRENT",
            "SET VOLTAGE",
            "CURRENT LIMIT",
            "VOLTAGE",
            "CURRENT",
            "POWER",
            "MAX",
            "PROTOCOL",
            "BRIGHTNESS",
            "ROTATION",
            "ENERGY MEMORY",
            "TIMER",
            "CALIBRATION",
            "AUTO",
            "NORMAL",
            "FLIPPED",
        ];
        for label in labels {
            for ch in translate(label).unwrap().chars() {
                assert!(glyph(ch).is_some(), "Missing {ch} in {label}");
            }
        }
    }
}
