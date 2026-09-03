use embedded_can::Frame;

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
mod deltaq_icl1500_can_messages {
    include!(concat!(
        env!("OUT_DIR"),
        "/deltaq_icl1500_can_messages.rs"
    ));
}

pub use deltaq_icl1500_can_messages::CanError;
pub use deltaq_icl1500_can_messages::DeltaQRpdo10x20a;
pub use deltaq_icl1500_can_messages::DeltaQRpdo10x20aBattChargeCycleType;
pub use deltaq_icl1500_can_messages::DeltaQRpdo10x20aBatteryStatus;
pub use deltaq_icl1500_can_messages::DeltaQRpdo20x30a;

/// Send a charge command to the Delta-Q charger.
///
/// Sends two CAN frames:
/// - RPDO2 (0x30a): battery current, voltage, temperature
/// - RPDO1 (0x20a): SOC, charge cycle type, voltage request, current request, status
///
/// Pass `current == 0.0` to disable the charger.
pub async fn send_command(
    can_socket_tx: &tokio_socketcan::CANSocket,
    volts: f32,
    current: f32,
    temperature: f32,
    soc: u8,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let frame = DeltaQRpdo20x30a::new(current, volts, temperature)?;
    let id: u32 = match frame.id() {
        embedded_can::Id::Standard(standard_id) => standard_id.as_raw() as u32,
        embedded_can::Id::Extended(extended_id) => extended_id.as_raw(),
    };
    let raw_frame = tokio_socketcan::CANFrame::new(id, frame.raw(), false, false)?;
    can_socket_tx.write_frame(raw_frame)?.await?;

    let batt_charge_cycle_type = if current == 0.0 {
        DeltaQRpdo10x20aBattChargeCycleType::NoActiveCycle
    } else {
        DeltaQRpdo10x20aBattChargeCycleType::Charge
    };

    let battery_status = if current == 0.0 {
        DeltaQRpdo10x20aBatteryStatus::Disabled
    } else {
        DeltaQRpdo10x20aBatteryStatus::Enabled
    };

    let frame = DeltaQRpdo10x20a::new(soc, batt_charge_cycle_type, volts, current, battery_status)?;
    let id: u32 = match frame.id() {
        embedded_can::Id::Standard(standard_id) => standard_id.as_raw() as u32,
        embedded_can::Id::Extended(extended_id) => extended_id.as_raw(),
    };
    let raw_frame = tokio_socketcan::CANFrame::new(id, frame.raw(), false, false)?;
    can_socket_tx.write_frame(raw_frame)?.await?;

    Ok(())
}
