use crate::qsl_context::QSLContext;
use crate::qsl_template::EyeballRecordTemplate;
use crate::qsl_type::{Mode, QSL};
use askama::Template;
use cursive::reexports::log;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

const TYPST_TEMPLATE: &str = include_str!("../templates/template_typst.typ");
pub(crate) struct QSLManager {
    pub context: QSLContext,
    pub(crate) split_page_size: i64,
    callsign: String,
    pub page: usize,
    max_page: usize,
    number_of_record: usize,
}

impl QSLManager {
    pub fn new(context: QSLContext, split_page_size: i64) -> Result<Self, String> {
        let callsign = match context.get_callsign() {
            Ok(cs) => cs,
            Err(err) => return Err(format!("Could not read callsign: {err}")),
        };
        let number_of_record = context.get_qsl_count()? as usize;
        let max_page = (number_of_record / split_page_size as usize + 1) - 1;

        Ok(QSLManager {
            context,
            split_page_size,
            callsign,
            page: 0,
            max_page,
            number_of_record,
        })
    }
    pub fn callsign(&self) -> &String {
        &self.callsign
    }

    pub fn max_page(&self) -> usize {
        self.max_page
    }

    pub fn number_of_record(&self) -> usize {
        self.number_of_record
    }
    pub fn fetch_shown_qsl(&mut self) -> Vec<QSL> {
        self.number_of_record = self.context.get_qsl_count().unwrap() as usize;
        log::debug!(
            "QSLManager::fetch_qsl: number of the record is {}",
            self.number_of_record
        );
        self.max_page = (self.number_of_record / self.split_page_size as usize + 1) as usize - 1;
        log::debug!("QSLManager::fetch_qsl: max_page is {}", self.max_page);

        if self.page >= self.max_page {
            self.page = self.max_page
        }

        self.context
            .get_qsl_page(self.split_page_size, self.page as i64)
            .unwrap()
    }

    pub fn output_typst(&self, file: &mut File) -> Result<(), String> {
        match file.write_all(format!("#let callsign = \"{}\"\n", self.callsign).as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        match file.write_all("#let log_data = (".as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        let total_pages = self.max_page() + 1;
        println!("There are {} pages.", total_pages);
        for i in 0..total_pages {
            match self.context.get_qsl_page(self.split_page_size, i as i64) {
                Ok(qsl_records) => {
                    println!("Page {} have {} records.", i, qsl_records.len());
                    for qsl in qsl_records {
                        if qsl.mode != Mode::EYEBALL {
                            match file.write_all(qsl.fmt_typst().as_bytes()) {
                                Ok(_) => {}
                                Err(e) => {
                                    return Err(format!("{}", e));
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(format!("Error on writing qsl record: {}", e));
                }
            }
        }

        match file.write_all(")\n".as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        match file.write_all(TYPST_TEMPLATE.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        Ok(())
    }

    pub fn output_html(&self, file_folder: &Path) -> Result<(), String> {
        let mut eyeball = Vec::<QSL>::new();

        // First, Eyeball page
        let record_count = self.context.get_eyeball_qsl_count()?;
        let total_pages = record_count / self.split_page_size + 1;

        for i in 0..total_pages {
            let qsl_records = self
                .context
                .get_eyeball_qsl_page(self.split_page_size, i as i64)?;
            println!("Page {} have {} records.", i, qsl_records.len());
            for qsl in qsl_records {
                if qsl.mode == Mode::EYEBALL {
                    eyeball.push(qsl.clone());
                }
            }
        }

        let template = EyeballRecordTemplate {
            callsign: &self.callsign,
            records: &eyeball,
        };

        let html_content = match template.render() {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Error on rendering qsl record: {}", e));
            }
        };
        let file_path = file_folder.join("eyeball.html");

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file_path)
        {
            Ok(mut file) => match file.write_all(html_content.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(format!("Error on writing eyeball.html: {}", e));
                }
            },
            Err(e) => {
                return Err(format!("Error on opening eyeball.html: {}", e));
            }
        };

        Ok(())
    }
}
