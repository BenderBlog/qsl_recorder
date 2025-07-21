use crate::qsl_context::Context;
use crate::qsl_type::QSL;
use cursive::reexports::log;

const SPLIT_PAGE_SIZE: i64 = 18;

pub(crate) struct QSLManager {
    pub context: Context,
    callsign: String,
    pub page: usize,
    max_page: usize,
    number_of_record: usize,
}

impl QSLManager {
    pub fn new(context: Context) -> Result<Self, String> {
        let callsign = match context.get_callsign() {
            Ok(cs) => cs,
            Err(err) => return Err(format!("Could not read callsign: {err}")),
        };

        Ok(QSLManager {
            context,
            callsign,
            page: 0,
            max_page: 0,
            number_of_record: 0,
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
        self.max_page = (self.number_of_record / SPLIT_PAGE_SIZE as usize + 1) as usize - 1;
        log::debug!("QSLManager::fetch_qsl: max_page is {}", self.max_page);

        if self.page >= self.max_page {
            self.page = self.max_page
        }

        self.context
            .get_qsl_page(SPLIT_PAGE_SIZE, self.page as i64)
            .unwrap()
    }
}
