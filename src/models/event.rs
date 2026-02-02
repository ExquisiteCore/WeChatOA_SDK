use serde::Deserialize;

/// Subscribe/Unsubscribe event.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscribeEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    /// Event type (subscribe/unsubscribe)
    pub event: String,
    /// Event key (QR code scene value for scan subscribe)
    pub event_key: Option<String>,
    /// QR code ticket (for scan subscribe)
    pub ticket: Option<String>,
}

/// QR code scan event (user already subscribed).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub event: String,
    /// Scene value from QR code
    pub event_key: String,
    /// QR code ticket
    pub ticket: String,
}

/// Location report event.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub event: String,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Precision
    pub precision: f64,
}

/// Menu click event (CLICK type).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MenuClickEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub event: String,
    /// Custom menu key value
    pub event_key: String,
}

/// Menu view event (VIEW type - user clicks menu link).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MenuViewEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub event: String,
    /// URL that was clicked
    pub event_key: String,
}

/// Template message send job finish event.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateSendJobFinishEvent {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub event: String,
    /// Message ID
    #[serde(rename = "MsgID")]
    pub msg_id: i64,
    /// Status: "success", "failed:user block", "failed: system failed"
    pub status: String,
}

/// Helper to peek at event type.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct EventTypePeek {
    pub event: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_parse_subscribe_event() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[event]]></MsgType>
<Event><![CDATA[subscribe]]></Event>
</xml>"#;

        let event: SubscribeEvent = from_str(xml).unwrap();
        assert_eq!(event.to_user_name, "gh_test");
        assert_eq!(event.from_user_name, "oUser123");
        assert_eq!(event.event, "subscribe");
        assert!(event.event_key.is_none());
    }

    #[test]
    fn test_parse_menu_click_event() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[event]]></MsgType>
<Event><![CDATA[CLICK]]></Event>
<EventKey><![CDATA[menu_key_1]]></EventKey>
</xml>"#;

        let event: MenuClickEvent = from_str(xml).unwrap();
        assert_eq!(event.event, "CLICK");
        assert_eq!(event.event_key, "menu_key_1");
    }

    #[test]
    fn test_parse_location_event() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[event]]></MsgType>
<Event><![CDATA[LOCATION]]></Event>
<Latitude>23.137466</Latitude>
<Longitude>113.352425</Longitude>
<Precision>119.385040</Precision>
</xml>"#;

        let event: LocationEvent = from_str(xml).unwrap();
        assert!((event.latitude - 23.137466).abs() < 0.0001);
        assert!((event.longitude - 113.352425).abs() < 0.0001);
    }
}
