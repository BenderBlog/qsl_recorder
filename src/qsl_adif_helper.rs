use chrono::{DateTime, Local};

pub fn adif_generate_line(k: &str, v: &str) -> String {
    let len = v.len();
    format!("<{k}:{len}>{v} ")
}

pub fn adif_generate_header(datetime: &DateTime<Local>) -> String {
    let mut str = adif_generate_line("ADIF_VER", "3.1.4");
    str.push_str("\n");
    str.push_str(&adif_generate_line("PROGRAMID", "BenderBlo1g qsl_recorder"));
    str.push_str("\n");
    str.push_str(&adif_generate_line("PROGRAMVERSION", "Rolling-20240728"));
    str.push_str("\n");
    str.push_str(&adif_generate_line(
        "CREATED_TIMESTAMP",
        datetime.format("%Y%m%d %H%M00").to_string().as_str(),
    ));
    str.push_str("\n");
    str.push_str("<EOH>\n");
    str
}
