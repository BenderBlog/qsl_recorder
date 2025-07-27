use chrono::NaiveDateTime;
use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum Usage {
    HTML,
    TYPST,
    ADIF,
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

    pub fn get_band<'a>(&self) -> Result<impl AsRef<str>, String> {
        let split = match (self.freq.as_ref()) {
            None => {
                return Err("Freq is none".to_string());
            }
            Some(str) => str.split("/").collect::<Vec<_>>(),
        };

        if split.is_empty() {
            return Err(
                "Format not match, each parameter is divided with '/' with no spacebar surrounded."
                    .to_string(),
            );
        }

        match split.first().unwrap().parse::<f32>() {
            Ok(freq_mhz) => {
                if (0.1357..=0.1378).contains(&freq_mhz) {
                    Ok("2190m")
                } else if (0.472..=0.479).contains(&freq_mhz) {
                    Ok("630m")
                } else if (0.501..=0.504).contains(&freq_mhz) {
                    Ok("560m")
                } else if (1.8..=2.0).contains(&freq_mhz) {
                    Ok("160m")
                } else if (3.5..=4.0).contains(&freq_mhz) {
                    Ok("80m")
                } else if (5.06..=5.45).contains(&freq_mhz) {
                    Ok("60m")
                } else if (7.0..=7.3).contains(&freq_mhz) {
                    Ok("40m")
                } else if (10.1..=10.15).contains(&freq_mhz) {
                    Ok("30m")
                } else if (14.0..=14.35).contains(&freq_mhz) {
                    Ok("20m")
                } else if (18.068..=18.168).contains(&freq_mhz) {
                    Ok("17m")
                } else if (21.0..=21.45).contains(&freq_mhz) {
                    Ok("15m")
                } else if (24.890..=24.99).contains(&freq_mhz) {
                    Ok("12m")
                } else if (28.0..=29.7).contains(&freq_mhz) {
                    Ok("10m")
                } else if (40.0..=45.0).contains(&freq_mhz) {
                    Ok("8m")
                } else if (50.0..=54.0).contains(&freq_mhz) {
                    Ok("6m")
                } else if (54.000001..=69.9).contains(&freq_mhz) {
                    Ok("5m")
                } else if (70.0..=71.0).contains(&freq_mhz) {
                    Ok("4m")
                } else if (144.0..=148.0).contains(&freq_mhz) {
                    Ok("2m")
                } else if (222.0..=225.0).contains(&freq_mhz) {
                    Ok("1.25m")
                } else if (420.0..=450.0).contains(&freq_mhz) {
                    Ok("70cm")
                } else if (902.0..=928.0).contains(&freq_mhz) {
                    Ok("33cm")
                } else if (1240.0..=1300.0).contains(&freq_mhz) {
                    Ok("23cm")
                } else if (2300.0..=2450.0).contains(&freq_mhz) {
                    Ok("13cm")
                } else if (3300.0..=3500.0).contains(&freq_mhz) {
                    Ok("9cm")
                } else if (5650.0..=5925.0).contains(&freq_mhz) {
                    Ok("6cm")
                } else if (10000.0..=10500.0).contains(&freq_mhz) {
                    Ok("3cm")
                } else if (24000.0..=24250.0).contains(&freq_mhz) {
                    Ok("1.25cm")
                } else if (47000.0..=47200.0).contains(&freq_mhz) {
                    Ok("6mm")
                } else if (75500.0..=81000.0).contains(&freq_mhz) {
                    Ok("4mm")
                } else if (119980.0..=123000.0).contains(&freq_mhz) {
                    Ok("2.5mm")
                } else if (134000.0..=149000.0).contains(&freq_mhz) {
                    Ok("2mm")
                } else if (241000.0..=250000.0).contains(&freq_mhz) {
                    Ok("1mm")
                } else if (300000.0..=7500000.0).contains(&freq_mhz) {
                    Ok("submm")
                } else {
                    Err(format!(
                        "Cannot parse {freq_mhz} because it is not in the standard's band range, as shown in III.B.4."
                    ).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
