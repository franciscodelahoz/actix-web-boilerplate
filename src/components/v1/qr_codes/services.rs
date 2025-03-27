use qrcode::{ QrCode, render };

use crate::libraries::qr_codes::{ build_qr_wifi_code, QRWifiCodeParams };

use super::schemas::QrWifiQuery;

pub async fn qr_wifi_handler(query: QrWifiQuery) -> String {
    let params = QRWifiCodeParams {
        ssid: query.ssid.clone(),
        encryption: query.encryption.clone(),
        password: query.password.clone(),
        hidden: query.hidden.unwrap_or(false),
    };

    let wifi_payload = build_qr_wifi_code(&params);

    let code = QrCode::new(wifi_payload).expect("Failed to create QR");

    let qr_svg = code
        .render::<render::svg::Color>()
        .build();

    return qr_svg;
}
