# ffbeast-wheel-api

A native Rust HID library for communicating with the **[FFBeast](https://ffbeast.github.io/) steering wheel controller**.

This is a full port of the TypeScript WebHID library
[`@shubham0x13/ffbeast-wheel-webhid-api`](https://github.com/shubham0x13/ffbeast-wheel-webhid-api),
rewritten for native environments using the [`hidapi`](https://crates.io/crates/hidapi) crate.
The HID report layout and protocol are identical to the C++ reference implementation.

---

## Features

- **Connect** to the wheel by USB VID/PID (no path guessing needed)
- **Read** all settings groups: effect, hardware, GPIO extension, ADC extension
- **Read** real-time device state (position, torque, firmware info)
- **Write** any individual setting field using the same typed dispatch as the original TypeScript API
- **Direct force control** — spring / constant / periodic forces + force-drop
- **Firmware activation** — send a `XXXXXXXX-XXXXXXXX-XXXXXXXX` license key
- **One-shot commands** — reboot, save-and-reboot, DFU mode, reset center
- **Blocking event loop** via `listen(callback)` — stop by returning `false`

---

## Requirements

| Dependency | Version |
|------------|---------|
| Rust | 1.65+ (2021 edition) |
| `hidapi` | 2.x |
| `thiserror` | 1.x |

On Linux, `libhidapi-hidraw` (or `libhidapi-libusb`) is required at link time.
On macOS and Windows, `hidapi` builds against the platform SDK and needs no extra packages.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ffbeast-wheel-api = { path = "../ffbeast-wheel-api-rs" }
```

_(Publish to crates.io pending — use a path or git dependency for now.)_

---

## Usage

### Connect and read settings

```rust
use ffbeast_wheel_api::WheelApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wheel = WheelApi::connect()?;

    let settings = wheel.read_all_settings()?;
    let e = &settings.effects;
    println!("Motion range : {} °", e.motion_range);
    println!("Total strength: {}%", e.total_effect_strength);
    println!("Static damping: {}%", e.static_dampening_strength);

    let h = &settings.hardware;
    println!("Encoder CPR : {}", h.encoder_cpr);
    println!("Force enabled: {}", h.force_enabled);

    Ok(())
}
```

### Streaming device state

```rust
use ffbeast_wheel_api::WheelApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wheel = WheelApi::connect()?;

    wheel.listen(|state| {
        println!(
            "pos={:6}  torque={:6}  deg={:8.2?}  norm={:6.1}%",
            state.position,
            state.torque,
            state.position_degrees,
            state.torque_normalized,
        );
        true // return false to stop
    })?;

    Ok(())
}
```

### Read a single state report (non-blocking, 100 ms timeout)

```rust
let mut wheel = WheelApi::connect()?;
if let Some(state) = wheel.read_state()? {
    println!("position: {}", state.position);
}
```

### Write a setting

```rust
use ffbeast_wheel_api::{WheelApi, SettingField};

let mut wheel = WheelApi::connect()?;

// Set motion range to 900 degrees
wheel.send_setting(SettingField::MotionRange, 0, 900)?;

// Save and reboot to apply
wheel.save_and_reboot()?;
```

Typed convenience helpers are also provided:

```rust
wheel.send_u16_setting(SettingField::MotionRange, 0, 900)?;
wheel.send_u8_setting(SettingField::TotalEffectStrength, 0, 80)?;
wheel.send_i8_setting(SettingField::ForceDirection, 0, -1)?;
wheel.send_float_setting(SettingField::SomeFloatField, 0, 1.5)?;
```

### Direct force control

```rust
use ffbeast_wheel_api::{WheelApi, DirectControl};

let wheel = WheelApi::connect()?;

wheel.send_direct_control(&DirectControl {
    spring_force: 3_000,    // ±10 000 range
    constant_force: 0,
    periodic_force: -1_500,
    force_drop: 0,          // 0–100%
})?;
```

### Firmware activation

```rust
let wheel = WheelApi::connect()?;
wheel.send_firmware_activation("AABBCCDD-11223344-DEADBEEF")?;
```

### Device commands

```rust
wheel.save_and_reboot()?;    // save settings to flash, then reboot
wheel.reboot_controller()?;  // reboot without saving
wheel.switch_to_dfu()?;      // enter DFU firmware-update mode
wheel.reset_wheel_center()?; // set current position as center
```

---

## HID report layout

All communication uses the `GenericInputOutput` report ID (`0xA3`).

| Direction | Byte 0 | Byte 1 | Bytes 2+ |
|-----------|--------|--------|----------|
| Output (command) | `0xA3` | command byte | — |
| Output (setting) | `0xA3` | `0x14` | field, index, value (LE) |
| Output (direct control) | `0xA3` | `0x10` | spring, constant, periodic (i16 LE), drop (u8) |
| Output (activation) | `0xA3` | `0x13` | 3 × u32 LE |
| Input (state) | `0xA3` | `release_type` | major, minor, patch, registered, position (i16 LE), torque (i16 LE) |

Feature reports (settings reads) use their own report IDs (`0x21`, `0x22`, `0x25`, `0xA1`, `0xA2`).

---

## Testing

Unit tests cover all pure logic:

```sh
cargo test
```

Hardware-dependent tests (connect, read, listen) require the FFBeast controller to be
plugged in. They are **not** included in the automated test suite because they cannot
run in CI. You can add your own integration test behind `#[ignore]` and run it with:

```sh
cargo test -- --ignored
```

---

## License

MIT — see [LICENSE](LICENSE).

Original TypeScript implementation © Shubham Patel.
