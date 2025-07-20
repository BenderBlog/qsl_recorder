use crate::qsl_manager::QSLManager;
use crate::qsl_type::Mode;
use crate::qsl_type::QSL;
use chrono::{Datelike, Timelike};
use cursive::reexports::log;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Dialog, ListView, OnEventView, SelectView};
use cursive::views::{EditView, LinearLayout, TextView};
use cursive::{Cursive, event};
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum UIColumn {
    Callsign,
    Mode,
    Datetime,
    Note,
}

impl UIColumn {
    pub fn as_str(&self) -> &str {
        match *self {
            UIColumn::Callsign => "Callsign",
            UIColumn::Mode => "Mode",
            UIColumn::Datetime => "Datetime",
            UIColumn::Note => "Note",
        }
    }
}

impl TableViewItem<UIColumn> for QSL {
    fn to_column(&self, column: UIColumn) -> String {
        match column {
            UIColumn::Note => self.note.clone().unwrap_or("".parse().unwrap()),
            UIColumn::Callsign => self.call_number.clone(),
            UIColumn::Mode => format!("{:?}", self.mode),
            UIColumn::Datetime => self.datetime.to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: UIColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            UIColumn::Note => self
                .note
                .clone()
                .unwrap_or("".parse().unwrap())
                .cmp(&other.note.clone().unwrap_or("".parse().unwrap())),
            UIColumn::Callsign => self.call_number.cmp(&other.call_number),
            UIColumn::Mode => format!("{:?}", self.mode).cmp(&format!("{:?}", other.mode)),
            UIColumn::Datetime => self.datetime.cmp(&other.datetime),
        }
    }
}

pub fn show_qsl_table(s: &mut Cursive) {
    s.pop_layer();

    let mut table = TableView::<QSL, UIColumn>::new()
        .column(UIColumn::Datetime, UIColumn::Datetime.as_str(), |c| {
            c.width_percent(35)
        })
        .column(UIColumn::Callsign, UIColumn::Callsign.as_str(), |c| {
            c.width_percent(20)
        })
        .column(UIColumn::Mode, UIColumn::Mode.as_str(), |c| {
            c.width_percent(10)
        })
        .column(UIColumn::Note, UIColumn::Note.as_str(), |c| {
            c.ordering(Ordering::Greater).width_percent(35)
        });

    let qslmanager = s.user_data::<QSLManager>().unwrap();

    let record = qslmanager.fetch_shown_qsl();
    log::debug!(
        "qsl_ui::show_qsl_table: inserting {} item(s).",
        record.len()
    );
    table.set_items(record);

    let callsign = qslmanager.callsign();
    let page = qslmanager.page;
    let max_page = qslmanager.max_page();
    let number_of_record = qslmanager.number_of_record();

    table.set_on_submit(|siv: &mut Cursive, _row: usize, index: usize| {
        let qsl = siv
            .call_on_name("table", move |table: &mut TableView<QSL, UIColumn>| {
                table.borrow_item(index).unwrap().clone()
            })
            .unwrap();

        edit_record_dialog(siv, Some(qsl));
    });

    let dialog = Dialog::around(
        OnEventView::new(table.with_name("table").min_size((70, 20)))
            .on_event(event::Key::Left, |s| previous_page(s))
            .on_event(event::Key::Right, |s| next_page(s)),
    )
    .title(format!(
        "{} page {} / {} total {}",
        callsign,
        page + 1,
        max_page + 1,
        number_of_record
    ));
    s.add_layer(dialog);

    fn next_page(s: &mut Cursive) {
        if let Some(qslmanager) = s.user_data::<QSLManager>() {
            if qslmanager.page < qslmanager.max_page() {
                qslmanager.page += 1;
                show_qsl_table(s);
            }
        }
    }

    fn previous_page(s: &mut Cursive) {
        if let Some(qslmanager) = s.user_data::<QSLManager>() {
            if qslmanager.page > 0 {
                qslmanager.page -= 1;
                show_qsl_table(s);
            }
        }
    }
}

pub fn edit_record_dialog(s: &mut Cursive, qsl: Option<QSL>) {
    let is_new = qsl.is_none();

    let id = if is_new { 0 } else { qsl.as_ref().unwrap().id };
    let mut call_number = "".to_string();
    let mode = if is_new {
        Mode::EYEBALL
    } else {
        qsl.as_ref().unwrap().mode.clone()
    };
    let mut freq = "".to_string();

    let mut year = "".to_string();
    let mut month = "".to_string();
    let mut day = "".to_string();
    let mut hour = "".to_string();
    let mut minute = "".to_string();

    let mut rst_me = "".to_string();
    let mut qth_me = "".to_string();
    let mut rig_me = "".to_string();
    let mut watt_me = "".to_string();
    let mut ant_me = "".to_string();
    let mut rst_counterpart = "".to_string();
    let mut qth_counterpart = "".to_string();
    let mut rig_counterpart = "".to_string();
    let mut watt_counterpart = "".to_string();
    let mut ant_counterpart = "".to_string();
    let mut note = "".to_string();

    if !is_new {
        let qsl = qsl.unwrap();
        call_number = qsl.call_number.clone();
        freq = qsl.freq.unwrap_or_default();

        let datetime = qsl.datetime;
        year = datetime.year().to_string().clone();
        month = datetime.month().to_string();
        day = datetime.day().to_string();
        hour = datetime.hour().to_string();
        minute = datetime.minute().to_string();

        rst_me = qsl.rst_me.unwrap_or_default();
        qth_me = qsl.qth_me.unwrap_or_default();
        rig_me = qsl.rig_me.unwrap_or_default();
        watt_me = qsl.watt_me.unwrap_or_default().to_string();
        ant_me = qsl.ant_me.unwrap_or_default();

        rst_counterpart = qsl.rst_counterpart.unwrap_or_default();
        qth_counterpart = qsl.qth_counterpart.unwrap_or_default();
        rig_counterpart = qsl.rig_counterpart.unwrap_or_default();
        watt_counterpart = qsl.watt_counterpart.unwrap_or_default().to_string();
        ant_counterpart = qsl.ant_counterpart.unwrap_or_default();
        note = qsl.note.unwrap_or_default();
    }

    let mut widget = Dialog::text("Add qsl record.")
        .title("Add QSL Record")
        .content(
            ListView::new()
                // Each child is a single-line view with a label
                .child(
                    "Callsign",
                    EditView::new()
                        .content(call_number)
                        .with_name("call_number"),
                )
                .child(
                    "Datetime",
                    LinearLayout::horizontal()
                        .child(
                            EditView::new()
                                .content(year)
                                .with_name("year")
                                .fixed_width(5),
                        )
                        .child(TextView::new("-"))
                        .child(
                            EditView::new()
                                .content(month)
                                .with_name("month")
                                .fixed_width(3),
                        )
                        .child(TextView::new("-"))
                        .child(EditView::new().content(day).with_name("day").fixed_width(3))
                        .child(TextView::new(" "))
                        .child(
                            EditView::new()
                                .content(hour)
                                .with_name("hour")
                                .fixed_width(3),
                        )
                        .child(TextView::new(":"))
                        .child(
                            EditView::new()
                                .content(minute)
                                .with_name("minute")
                                .fixed_width(3),
                        ),
                )
                .child(
                    "Mode",
                    SelectView::<Mode>::new()
                        .popup()
                        .item(Mode::EYEBALL.to_string(), Mode::EYEBALL)
                        .item(Mode::FM.to_string(), Mode::FM)
                        .item(Mode::SSB.to_string(), Mode::SSB)
                        .item(Mode::CW.to_string(), Mode::CW)
                        .item(Mode::FTB.to_string(), Mode::FTB)
                        .item(Mode::OTHER.to_string(), Mode::OTHER)
                        .selected(mode as usize)
                        .with_name("mode"),
                )
                .child("Frequency", EditView::new().content(freq).with_name("freq"))
                .child(
                    "RST (Me)",
                    EditView::new().content(rst_me).with_name("rst_me"),
                )
                .child(
                    "QTH (Me)",
                    EditView::new().content(qth_me).with_name("qth_me"),
                )
                .child(
                    "Rig (Me)",
                    EditView::new().content(rig_me).with_name("rig_me"),
                )
                .child(
                    "Watt (Me)",
                    EditView::new().content(watt_me).with_name("watt_me"),
                )
                .child(
                    "Antenna (Me)",
                    EditView::new().content(ant_me).with_name("ant_me"),
                )
                .child(
                    "RST (Counterpart)",
                    EditView::new()
                        .content(rst_counterpart)
                        .with_name("rst_counterpart"),
                )
                .child(
                    "QTH (Counterpart)",
                    EditView::new()
                        .content(qth_counterpart)
                        .with_name("qth_counterpart"),
                )
                .child(
                    "Rig (Counterpart)",
                    EditView::new()
                        .content(rig_counterpart)
                        .with_name("rig_counterpart"),
                )
                .child(
                    "Watt (Counterpart)",
                    EditView::new()
                        .content(watt_counterpart)
                        .with_name("watt_counterpart"),
                )
                .child(
                    "Antenna (Counterpart)",
                    EditView::new()
                        .content(ant_counterpart)
                        .with_name("ant_counterpart"),
                )
                .child("Note", EditView::new().content(note).with_name("note"))
                .scrollable(),
        )
        .button("Submit", move |s| {
            log::debug!("Ready to create new qsl record...");

            let call_number = s.call_on_name("call_number", |view: &mut EditView| {
                view.get_content().to_string()
            });

            if call_number.is_none() || call_number.clone().is_some_and(|x| x.is_empty()) {
                show_error_dialog(
                    s,
                    "call_number cannot be empty! You can write NOCALL if you insist.",
                );
                return;
            }
            log::debug!("Callsign initialized...");

            let year = match s
                .call_on_name("year", |view: &mut EditView| view.get_content().to_string())
                .unwrap()
                .parse::<i32>()
            {
                Ok(d) => d,
                Err(e) => {
                    show_error_dialog(s, &format!("Parse year failed: {e}"));
                    return;
                }
            };
            let month = match s
                .call_on_name("month", |view: &mut EditView| {
                    view.get_content().to_string()
                })
                .unwrap()
                .parse::<u32>()
            {
                Ok(d) => d,
                Err(e) => {
                    show_error_dialog(s, &format!("Parse month failed: {e}"));
                    return;
                }
            };
            let day = match s
                .call_on_name("day", |view: &mut EditView| view.get_content().to_string())
                .unwrap()
                .parse::<u32>()
            {
                Ok(d) => d,
                Err(e) => {
                    show_error_dialog(s, &format!("Parse day failed: {e}"));
                    return;
                }
            };
            let hour = match s
                .call_on_name("hour", |view: &mut EditView| view.get_content().to_string())
                .unwrap()
                .parse::<u32>()
            {
                Ok(d) => d,
                Err(e) => {
                    show_error_dialog(s, &format!("Parse hour failed: {e}"));
                    return;
                }
            };
            let minute = match s
                .call_on_name("minute", |view: &mut EditView| {
                    view.get_content().to_string()
                })
                .unwrap()
                .parse::<u32>()
            {
                Ok(d) => d,
                Err(e) => {
                    show_error_dialog(s, &format!("Parse minute failed: {e}"));
                    return;
                }
            };

            let datetime = match chrono::NaiveDate::from_ymd_opt(year, month, day) {
                Some(date) => match date.and_hms_opt(hour, minute, 0) {
                    Some(date) => date,
                    None => {
                        show_error_dialog(s, "Invalid time, check hour and minute is valid?");
                        return;
                    }
                },
                None => {
                    show_error_dialog(s, "Invalid date, check year, month and day is valid?");
                    return;
                }
            };
            log::debug!("Datetime initialized...");

            let watt_me = match string_parser(
                &s.call_on_name("watt_me", |view: &mut EditView| {
                    view.get_content().to_string()
                })
                .unwrap(),
            ) {
                None => None,
                Some(str) => match str.parse::<f32>() {
                    Ok(w) => Some(w),
                    Err(e) => {
                        show_error_dialog(s, &format!("Parse watt_me failed: {e}."));
                        return;
                    }
                },
            };
            let watt_counterpart = match string_parser(
                &s.call_on_name("watt_counterpart", |view: &mut EditView| {
                    view.get_content().to_string()
                })
                .unwrap(),
            ) {
                None => None,
                Some(str) => match str.parse::<f32>() {
                    Ok(w) => Some(w),
                    Err(e) => {
                        show_error_dialog(s, &format!("Parse watt_counterpart failed: {e}."));
                        return;
                    }
                },
            };

            let new_qsl = QSL {
                id: id.clone(),
                call_number: call_number.unwrap().clone(),

                mode: s
                    .call_on_name("mode", |view: &mut SelectView<Mode>| {
                        view.selection().unwrap().clone().deref().clone()
                    })
                    .unwrap(),
                freq: string_parser(
                    &s.call_on_name("freq", |view: &mut EditView| view.get_content().to_string())
                        .unwrap(),
                ),
                datetime,
                rst_me: string_parser(
                    &s.call_on_name("rst_me", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                qth_me: string_parser(
                    &s.call_on_name("qth_me", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                rig_me: string_parser(
                    &s.call_on_name("rig_me", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                watt_me,
                ant_me: string_parser(
                    &s.call_on_name("ant_me", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                rst_counterpart: string_parser(
                    &s.call_on_name("rst_counterpart", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                qth_counterpart: string_parser(
                    &s.call_on_name("qth_counterpart", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                rig_counterpart: string_parser(
                    &s.call_on_name("rig_counterpart", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                watt_counterpart,
                ant_counterpart: string_parser(
                    &s.call_on_name("ant_counterpart", |view: &mut EditView| {
                        view.get_content().to_string()
                    })
                    .unwrap(),
                ),
                note: string_parser(
                    &s.call_on_name("note", |view: &mut EditView| view.get_content().to_string())
                        .unwrap(),
                ),
            };
            log::debug!("Adding qsl record...");

            if let Some(qslmanager) = s.user_data::<QSLManager>() {
                log::debug!("Database connected...");
                if is_new {
                    match qslmanager.context.add_qsl(new_qsl) {
                        Ok(_) => {
                            log::debug!("QSL record added.");
                            qslmanager.fetch_shown_qsl();
                            s.pop_layer();
                            s.pop_layer();
                            show_qsl_table(s);
                            show_error_dialog(s, "QSL record added.");
                        }
                        Err(e) => {
                            log::debug!("QSL record not added with error: {e}");
                            show_error_dialog(s, &format!("Failed to add qsl record: {}", e));
                        }
                    }
                } else {
                    match qslmanager.context.update(new_qsl) {
                        Ok(_) => {
                            log::debug!("QSL record {id} updated.");
                            qslmanager.fetch_shown_qsl();
                            s.pop_layer();
                            s.pop_layer();
                            show_qsl_table(s);
                            show_error_dialog(s, &format!("QSL record {id} updated."));
                        }
                        Err(e) => {
                            log::debug!("QSL record {id} could not updated with error: {e}");
                            show_error_dialog(s, &format!("Failed to add update record {id}: {e}"));
                        }
                    }
                }
            } else {
                log::error!("Database not connected while trying to write record to database.");
                show_error_dialog(s, "Database error.");
            }
        });

    if !is_new {
        widget.add_button("Delete", move |s| {
            s.add_layer(
                Dialog::text("Are you sure?")
                    .title("Confirm deletion")
                    .button("No", |s| {
                        s.pop_layer();
                    })
                    .button("Yes", move |s| {
                        if let Some(qslmanager) = s.user_data::<QSLManager>() {
                            log::debug!("Database connected...");
                            match qslmanager.context.delete(id) {
                                Ok(_) => {
                                    log::debug!("QSL record {id} deleted.");
                                    s.pop_layer();
                                    s.pop_layer();
                                    show_qsl_table(s);
                                    show_error_dialog(s, &format!("QSL record {id} deleted."));
                                }
                                Err(e) => {
                                    log::debug!(
                                        "QSL record {id} could not deleted with error: {e}"
                                    );
                                    show_error_dialog(
                                        s,
                                        &format!("Failed to delete record {id}: {e}"),
                                    );
                                }
                            }
                        } else {
                            log::error!(
                                "Database not connected while trying to write record to database."
                            );
                            show_error_dialog(s, "Database error.");
                        }
                    }),
            );
        })
    };

    s.add_layer(widget.button("Cancel", |s| {
        s.pop_layer();
    }));
}

fn show_error_dialog(s: &mut Cursive, msg: &str) {
    s.add_layer(Dialog::text(msg).title("Error").button("OK", |s| {
        s.pop_layer();
    }));
}

fn string_parser(str: &String) -> Option<String> {
    if str.is_empty() {
        None
    } else {
        Some(str.to_string())
    }
}
