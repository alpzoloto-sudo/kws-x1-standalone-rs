# Ukrainian UI prototype / Український інтерфейс (перший етап)

This branch adds a **small, built-in 5×7 Ukrainian Cyrillic uppercase font** and a translation layer to the existing device UI, without touching measurement, PD policy, calibration, or power-control code.

## Перекладено

- Головна сторінка: **НАПРУГА**, **СТРУМ**, **ПОТУЖНІСТЬ**, **МАКС**, **ПРОТОКОЛ**.
- Налаштування: **НАЛАШТУВАННЯ**, **ЯСКРАВІСТЬ**, **ОБЕРТАННЯ**, **ПАМ'ЯТЬ**, **ТАЙМЕР**, **КАЛІБРУВАННЯ**.
- Режими повороту: **АВТО**, **ЗВИЧАЙНО**, **ПОВЕРНУТО**.

USB PD, PPS, AVS, QC, V/A/W and dynamic protocol fields deliberately remain unchanged. The numeric fonts retain their original size.

## Обмеження

**This is a prototype, not a fully localized or hardware-validated firmware release.** The compact 5×7 Cyrillic text is smaller than the stock 12–16 px English fonts. It requires checking legibility on the physical **240×135** panel. A future iteration can enlarge glyphs and localize help/error strings after verifying available flash space.

The firmware's existing GitHub Actions checks are expected to run on this pull request. Inspect their result and generated image before any flashing. Do not flash an artifact from an unverified build.

To preview screens on a PC (Rust stable):

```sh
cargo run -p firmware-core --example screens --locked -- ./ukrainian-preview
```

The image generator includes the **same firmware UI source** used for the device.

## CI verification

The firmware workflow runs on pushes to `feature/ukrainian-ui` and on pull requests. The resulting artifacts should include the firmware image and rendered screenshots only after successful validation.
