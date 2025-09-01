use utoipa::OpenApi;
use crate::libraries::schemas::qr_codes::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "QR Codes Generator API",
        version = "1.0.0",
        description = "A simple and fast API to generate QR codes for WiFi, SMS, email, phone numbers, VCard, and VCalendar events."
    ),
    paths(
        crate::controllers::qr_codes::qr_wifi_handler,
        crate::controllers::qr_codes::qr_sms_handler,
        crate::controllers::qr_codes::qr_email_handler,
        crate::controllers::qr_codes::qr_phone_handler,
        crate::controllers::qr_codes::qr_vcard_handler,
        crate::controllers::qr_codes::qr_vcalendar_handler,
        crate::controllers::monitoring::health_check
    ),
    components(schemas(QrWifiQuery, QrSMSQuery, QrEmailQuery, QrPhoneQuery, QrVCardQuery, QrVCalendarQuery)),
    tags(
        (name = "QR Codes", description = "Endpoints for generating QR codes"),
        (name = "Monitoring", description = "Monitoring endpoints")
    )
)]

pub struct ApiDoc;
