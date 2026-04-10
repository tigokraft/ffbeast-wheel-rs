//! `WheelApi` simple command methods: save, reboot, DFU, center reset.

use crate::enums::ReportData;
use crate::error::WheelError;

use super::WheelApi;

impl WheelApi {
    /// Saves current settings to flash and reboots the controller.
    pub fn save_and_reboot(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandSaveSettings)
    }

    /// Reboots the controller without saving settings.
    pub fn reboot_controller(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandReboot)
    }

    /// Switches the device to DFU mode for firmware updates.
    pub fn switch_to_dfu(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandDfuMode)
    }

    /// Sets the current wheel position as the center point.
    pub fn reset_wheel_center(&self) -> Result<(), WheelError> {
        self.send_generic_command(ReportData::CommandResetCenter)
    }
}
