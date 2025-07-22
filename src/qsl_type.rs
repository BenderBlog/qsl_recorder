use chrono::NaiveDateTime;
use std::fmt::Display;
#[derive(PartialEq, Eq)]
pub enum Usage {
    HTML,
    TYPST,
    UI,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl QSL {
    pub fn fmt_typst(&self) -> String {
        let date = self.datetime.date();
        let time = self.datetime.time().format("%H:%M");
        format!(
            r#"(
  call_number: "{}",
  mode: "{}",
  freq: "{}",
  date: "{}",
  time: "{}",
  rst_me: "{}",
  qth_me: "{}",
  rig_me: "{}",
  watt_me: "{}",
  ant_me: "{}",
  rst_counterpart: "{}",
  qth_counterpart: "{}",
  rig_counterpart: "{}",
  watt_counterpart: "{}",
  ant_counterpart: "{}",
  note: "{}",
),"#,
            self.call_number,
            self.mode,
            self.freq.as_ref().map_or("", |f| f),
            date,
            time,
            self.rst_me.as_ref().map_or("", |r| r),
            self.qth_me.as_ref().map_or("", |q| q),
            self.rig_me.as_ref().map_or("", |r| r),
            self.watt_me.map_or("".to_string(), |w| w.to_string()),
            self.ant_me.as_ref().map_or("", |a| a),
            self.rst_counterpart.as_ref().map_or("", |r| r),
            self.qth_counterpart.as_ref().map_or("", |q| q),
            self.rig_counterpart.as_ref().map_or("", |r| r),
            self.watt_counterpart
                .map_or("".to_string(), |w| w.to_string()),
            self.ant_counterpart.as_ref().map_or("", |a| a),
            self.note.as_ref().map_or("", |n| n)
        )
    }
}
