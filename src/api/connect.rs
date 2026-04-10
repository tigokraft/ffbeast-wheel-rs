//! `WheelApi::connect` — device enumeration and connection setup.

use hidapi::HidApi;

use crate::constants::{PID, VID};
use crate::error::WheelError;

use super::WheelApi;

impl WheelApi {
    /// Connects to the first FFBeast wheel found on vendor interface 0.
    ///
    /// Caches the motion range from effect settings immediately after
    /// connecting so that [`WheelApi::position_degrees`] is populated
    /// from the very first state report.
    ///
    /// # Errors
    ///
    /// Returns [`WheelError::DeviceNotFound`] if no matching device is found,
    /// or [`WheelError::Hid`] for any underlying HID error.
    pub fn connect() -> Result<Self, WheelError> {
        let api = HidApi::new()?;

        let path = api
            .device_list()
            .find(|d| {
                d.vendor_id() == VID && d.product_id() == PID && d.interface_number() == 0
            })
            .ok_or(WheelError::DeviceNotFound)?
            .path()
            .to_owned();

        let device = api.open_path(&path)?;

        let mut wheel = Self {
            device,
            cached_motion_range: None,
            last_state: None,
        };

        // Cache motion range so position-in-degrees is available immediately.
        if let Ok(s) = wheel.read_effect_settings() {
            wheel.cached_motion_range = Some(s.motion_range);
        }

        Ok(wheel)
    }
}
