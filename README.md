# paperanywhere-devices

Canonical board / panel catalog for the [paperanywhere](https://github.com/paperanywhere)
stack. Single source of truth for "what panels and integrated boards exist."

Consumed by both:

- **[paperanywhere/backend](https://github.com/paperanywhere/backend)** — seeds
  the `panel_models` table at migration time. The dashboard's device-creation
  wizard lists this catalog.
- **[paperanywhere/firmware](https://github.com/paperanywhere/firmware)** —
  composes per-board GPIO pin maps on top of the shared metadata. Each
  `boards/<name>.rs` is a one-liner that pulls a `Board` const from here
  and layers on the pins.

## What lives here

Shared metadata that both halves need to agree on:

- Vendor / model name / panel kind (bare panel vs integrated board)
- Physical dimensions / supported color modes / packing kind
- Capability flags (battery, buttons, sensors, buzzer, SD card)
- MCU family + Cargo feature flag for integrated boards
- Datasheet URL
- Stable numeric `id` (matches `panel_models.id`)

## What does *not* live here

Stays in the firmware repo:

- GPIO pin numbers (SCK / MOSI / CS / DC / RST / BUSY)
- Panel controller IC choice (UC8179 / IT8951 / UC8159)
- Per-controller quirks (`panel_data_inverted`, etc.)

## Adding a new board

1. Add a new `pub const` entry at the bottom of `src/lib.rs` with the next
   available `id`. Never reuse retired ids — the value lands in NVS on
   real devices.
2. Append it to `CATALOG`.
3. If it's an integrated board, follow up with a `boards/<slug>.rs` in
   the firmware repo + a matching Cargo feature flag.
4. Run `cargo test` — the test suite asserts uniqueness of ids/slugs,
   that the default color mode is in the supported list, and that
   integrated boards have a firmware target.

## License

MIT OR Apache-2.0
