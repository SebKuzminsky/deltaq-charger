use embedded_can::Frame;

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
mod deltaq_xv3300_can_messages {
    include!(concat!(
        env!("OUT_DIR"),
        "/deltaq_xv3300_can_messages.rs"
    ));
}

#[allow(dead_code)]
mod deltaq_xv3300_sdos {
    include!(concat!(
        env!("OUT_DIR"),
        "/deltaq_xv3300_object_dictionary.rs"
    ));
}

use deltaq_xv3300_sdos::{SdoReadable, SdoWritable};

pub async fn set_can_listen_only(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
    val: bool,
) -> Result<(), zencan_client::SdoClientError> {
    let can_listen_only = deltaq_xv3300_sdos::CanListenOnlyMode0x2032 {
        value: if val { 1 } else { 0 },
    };
    can_listen_only.write(sdo_client).await
}

pub async fn get_can_listen_only(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
) -> Result<bool, zencan_client::SdoClientError> {
    let can_listen_only = deltaq_xv3300_sdos::CanListenOnlyMode0x2032::new(sdo_client).await?;
    if can_listen_only.value == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn set_ac_current_limit(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
    amps: f32,
) -> Result<(), zencan_client::SdoClientError> {
    let ac_current_limit = deltaq_xv3300_sdos::ACCurrentLimit0x2202 {
        value: (amps * 256.0) as u16,
    };
    ac_current_limit.write(sdo_client).await
}

pub async fn get_ac_current_limit(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
) -> Result<f32, zencan_client::SdoClientError> {
    let ac_current_limit = deltaq_xv3300_sdos::ACCurrentLimit0x2202::new(sdo_client).await?;
    Ok(ac_current_limit.value as f32 / 256.0)
}

/// Send a charge command to the Delta-Q charger.
///
/// Sends two CAN frames:
/// - RPDO2 (0x300 + NodeId): battery current, voltage, temperature
/// - RPDO1 (0x200 + NodeId): SOC, charge cycle type, voltage request, current request, status
///
/// Pass `current == 0.0` to disable the charger.
pub async fn send_command_xv3300(
    can_socket_tx: &tokio_socketcan::CANSocket,
    volts: f32,
    current: f32,
    soc: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    // Varta sends this:
    // Data: 01 50 00 33 3c 30 00 01
    use deltaq_xv3300_can_messages::DeltaQRpdo1;

    let (charge_control, battery_status) = if current > 0.001 {
        (0x01, 0x01)
    } else {
        (0x00, 0x00)
    };

    let frame = DeltaQRpdo1::new(charge_control, soc, 0, volts, current, battery_status)?;
    let id: u32 = match frame.id() {
        embedded_can::Id::Standard(standard_id) => standard_id.as_raw() as u32,
        embedded_can::Id::Extended(extended_id) => extended_id.as_raw(),
    };
    let raw_frame = tokio_socketcan::CANFrame::new(id, frame.raw(), false, false)?;
    can_socket_tx.write_frame(raw_frame)?.await?;
    Ok(())
}
