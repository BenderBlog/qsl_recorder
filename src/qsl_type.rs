use chrono::NaiveDateTime;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum Mode {
    EYEBALL,
    FM,
    SSB,
    CW,
    FTB,
    OTHER,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Mode::EYEBALL => "EYEBALL".to_string(),
            Mode::FM => "FM".to_string(),
            Mode::SSB => "SSB".to_string(),
            Mode::CW => "CW".to_string(),
            Mode::FTB => "FTB".to_string(),
            Mode::OTHER => "OTHER".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub struct QSL {
    pub(crate) id: i32,
    pub(crate) call_number: String,
    pub(crate) mode: Mode,
    pub(crate) freq: Option<String>,
    pub(crate) datetime: NaiveDateTime,
    pub(crate) rst_me: Option<String>,
    pub(crate) qth_me: Option<String>,
    pub(crate) rig_me: Option<String>,
    pub(crate) watt_me: Option<f32>,
    pub(crate) ant_me: Option<String>,
    pub(crate) rst_counterpart: Option<String>,
    pub(crate) qth_counterpart: Option<String>,
    pub(crate) rig_counterpart: Option<String>,
    pub(crate) watt_counterpart: Option<f32>,
    pub(crate) ant_counterpart: Option<String>,
    pub(crate) note: Option<String>,
}
