use crate::qsl_adif_helper::adif_generate_header;
use crate::qsl_context::QSLContext;
use crate::qsl_template::RecordTemplate;
use crate::qsl_type::QSL;
use askama::Template;
use chrono::Local;
use cursive::reexports::log;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

const TYPST_TEMPLATE: &str = include_str!("../templates/template.typ");
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

        let record_count = self.context.get_formal_qsl_count()?;
        let total_pages = record_count / self.split_page_size + 1;
        println!("There are {} pages.", total_pages);
        for i in 0..total_pages {
            match self
                .context
                .get_formal_qsl_page(self.split_page_size, i as i64)
            {
                Ok(qsl_records) => {
                    println!("Page {} have {} records.", i, qsl_records.len());
                    for qsl in qsl_records {
                        match file.write_all(qsl.fmt_typst().as_bytes()) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(format!("{}", e));
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

    pub fn output_adif(&self, file: &mut File) -> Result<(), String> {
        let datetime = Local::now();
        println!("ADIF file will be created at {datetime}");

        match file.write_all(adif_generate_header(&datetime).as_ref()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        let record_count = self.context.get_formal_qsl_count()?;
        let total_pages = record_count / self.split_page_size + 1;
        println!("There are {} pages.", total_pages);
        for i in 0..total_pages {
            match self
                .context
                .get_formal_qsl_page(self.split_page_size, i as i64)
            {
                Ok(qsl_records) => {
                    println!("Page {} have {} records.", i, qsl_records.len());
                    for qsl in qsl_records {
                        match file.write_all(qsl.fmt_adif().as_bytes()) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(format!("{}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(format!("Error on writing qsl record: {}", e));
                }
            }
        }

        Ok(())
    }

    pub fn output_html(&self, file_folder: &Path) -> Result<(), String> {
        let mut record_eyeball = Vec::<QSL>::new();
        let mut record_normal = Vec::<QSL>::new();

        // First, Eyeball page
        let record_count = self.context.get_eyeball_qsl_count()?;
        let total_pages = record_count / self.split_page_size + 1;

        for i in 0..total_pages {
            let mut qsl_records = self.context.get_eyeball_qsl_page(self.split_page_size, i)?;
            record_eyeball.append(&mut qsl_records);
        }

        let record_count = self.context.get_formal_qsl_count()?;
        let total_pages = record_count / self.split_page_size + 1;

        for i in 0..total_pages {
            let mut qsl_records = self.context.get_formal_qsl_page(self.split_page_size, i)?;
            record_normal.append(&mut qsl_records);
        }

        let template = RecordTemplate {
            callsign: &self.callsign,
            records_formal: &record_normal,
            records_eyeball: &record_eyeball,
        };

        match template.render() {
            Ok(html_content) => {
                let file_path = file_folder.join("index.html");

                match OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&file_path)
                {
                    Ok(mut file) => match file.write_all(html_content.as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(format!("Error on writing index.html: {}", e));
                        }
                    },
                    Err(e) => {
                        return Err(format!("Error on opening index.html: {}", e));
                    }
                };
            }
            Err(e) => {
                return Err(format!("Error on rendering qsl record: {}", e));
            }
        };

        Ok(())
    }
}
