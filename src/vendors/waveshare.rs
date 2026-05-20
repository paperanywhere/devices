//! Waveshare — bare e-paper panels driven by a user-wired ESP32 carrier.
//!
//! These entries describe the *panel* (resolution + controller-family
//! defaults), not a finished device. The firmware exposes generic
//! carrier builds (e.g. `board-generic-esp32s3-waveshare-75`) for the
//! sizes we actively support; other Waveshare panels are catalog-only
//! until a firmware target is wired.

use crate::{Board, BoardKind, ColorMode, McuFamily, PackingKind, PowerPolicy};

pub const WAVESHARE_42_BW: Board = Board {
    id: 5,
    slug: "waveshare-42-bw",
    vendor: "Waveshare",
    model: "4.2\" BW (UC8176)",
    kind: BoardKind::BarePanel,
    width_px: 400,
    height_px: 300,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/4.2inch_e-Paper_Module",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_42_3COLOR: Board = Board {
    id: 6,
    slug: "waveshare-42-3color",
    vendor: "Waveshare",
    model: "4.2\" 3-color (BW+Red)",
    kind: BoardKind::BarePanel,
    width_px: 400,
    height_px: 300,
    default_color_mode: ColorMode::MonoRed1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::MonoRed1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/4.2inch_e-Paper_Module_(B)",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_75_BW: Board = Board {
    id: 7,
    slug: "waveshare-75-bw",
    vendor: "Waveshare",
    model: "7.5\" BW (800x480)",
    kind: BoardKind::BarePanel,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/7.5inch_e-Paper_HAT",
    firmware_target: Some("board-generic-esp32s3-waveshare-75"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_73_ACEP: Board = Board {
    id: 8,
    slug: "waveshare-73-acep",
    vendor: "Waveshare",
    model: "7.3\" 7-color ACeP (Spectra 6)",
    kind: BoardKind::BarePanel,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Color7,
    supported_color_modes: &[ColorMode::Color7],
    packing_kind: PackingKind::AcepIndexed4bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/7.3inch_e-Paper_HAT_(F)",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};
