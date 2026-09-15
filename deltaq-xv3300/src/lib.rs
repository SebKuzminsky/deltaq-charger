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

pub async fn get_dc_current(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
) -> Result<f32, zencan_client::SdoClientError> {
    let dc_current = deltaq_xv3300_sdos::ChargingCurrent0x2002::new(sdo_client).await?;
    Ok(dc_current.value as f32 / 256.0)
}

pub async fn get_temperature(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
) -> Result<i8, zencan_client::SdoClientError> {
    let temperature_c = deltaq_xv3300_sdos::ChargerTemperature0x2050::new(sdo_client).await?;
    Ok(temperature_c.value)
}

pub async fn get_fan_speed(
    sdo_client: &mut zencan_client::SdoClient<
        zencan_client::common::SocketCanSender,
        zencan_client::common::SocketCanReceiver,
    >,
) -> Result<u16, zencan_client::SdoClientError> {
    let fan_speed_rpm = deltaq_xv3300_sdos::FanSpeed0x2031::new(sdo_client).await?;
    Ok(fan_speed_rpm.value)
}
