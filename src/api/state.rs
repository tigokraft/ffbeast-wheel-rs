//! `WheelApi` state-reading: single poll and blocking event loop.

use crate::constants::REPORT_SIZE;
use crate::enums::ReportType;
use crate::error::WheelError;
use crate::protocol::{convert_position_to_degrees, normalize_torque, StructReader};
use crate::types::{DeviceState, FirmwareVersion};

use super::WheelApi;

impl WheelApi {
    /// Reads a single device state report (100 ms timeout).
    ///
    /// Returns `Ok(None)` when no report arrives within the timeout.
    /// The received state is cached and accessible via [`WheelApi::last_state`].
    pub fn read_state(&mut self) -> Result<Option<DeviceState>, WheelError> {
        let mut buf = [0u8; REPORT_SIZE + 1];
        let n = self.device.read_timeout(&mut buf, 100)?;

        if n == 0 || buf[0] != ReportType::GenericInputOutput as u8 {
            return Ok(None);
        }

        // Parse from byte 1 (skipping report ID), mirroring DeviceStateTypeDef.
        let mut r = StructReader::new(&buf[1..]);

        let firmware_version = FirmwareVersion {
            release_type: r.u8()?,
            major: r.u8()?,
            minor: r.u8()?,
            patch: r.u8()?,
        };
        let is_registered = r.u8()?;
        let position = r.i16()?;
        let torque = r.i16()?;

        let position_degrees = self
            .cached_motion_range
            .map(|range| convert_position_to_degrees(position, range));

        let torque_normalized = normalize_torque(torque);

        let state = DeviceState {
            firmware_version,
            is_registered,
            position,
            torque,
            position_degrees,
            torque_normalized,
        };

        self.last_state = Some(state.clone());
        Ok(Some(state))
    }

    /// Blocking state-polling loop.
    ///
    /// Continuously reads device state reports and forwards each to
    /// `callback`. Return `true` from the callback to keep going, or `false`
    /// to stop cleanly.
    ///
    /// Any HID or parse error is propagated and terminates the loop.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ffbeast_wheel_api::WheelApi;
    /// let mut wheel = WheelApi::connect().unwrap();
    /// wheel.listen(|state| {
    ///     println!("pos {} torque {}", state.position, state.torque);
    ///     true
    /// }).unwrap();
    /// ```
    pub fn listen<F>(&mut self, mut callback: F) -> Result<(), WheelError>
    where
        F: FnMut(DeviceState) -> bool,
    {
        loop {
            match self.read_state()? {
                Some(state) => {
                    if !callback(state) {
                        return Ok(());
                    }
                }
                None => {} // Timeout — no report within 100 ms, keep polling.
            }
        }
    }
}
