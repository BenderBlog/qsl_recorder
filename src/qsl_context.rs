use crate::qsl_type::{Mode, QSL};
use cursive::reexports::log;
use rusqlite::{Connection, Error, Row, ToSql, params};

const NEW_DATABASE_QUERY: &str = r#"
BEGIN;

CREATE TABLE qsl (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    call_number TEXT NOT NULL,
    call_type INTEGER NOT NULL,
    freq TEXT,
    datetime TEXT NOT NULL,
    rst_me TEXT,
    rig_me TEXT,
    watt_me REAL,
    ant_me TEXT,
    qth_me TEXT,
    rst_counterpart TEXT,
    rig_counterpart TEXT,
    watt_counterpart REAL,
    ant_counterpart TEXT,
    qth_counterpart TEXT,
    note TEXT
);

CREATE TABLE setting (
    call_number TEXT
);

COMMIT;
"#;
const UPDATE_SETTING_QUERY: &str = "INSERT INTO setting(call_number) VALUES (?1);";
const READ_SETTING_QUERY: &str = "SELECT call_number FROM setting";
const CHECK_EXISTENCE_QUERY: &str = "SELECT 1 FROM qsl WHERE id = ?1";
const ADD_ELEMENT_QUERY: &str = r#"
INSERT INTO qsl (
    call_number,
    call_type,
    freq,
    datetime,
    rst_me,
    rig_me,
    watt_me,
    ant_me,
    qth_me,
    rst_counterpart,
    rig_counterpart,
    watt_counterpart,
    ant_counterpart,
    qth_counterpart,
    note
)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
"#;
const UPDATE_ELEMENT_QUERY: &str = r#"
UPDATE qsl
SET call_number = ?1, call_type = ?2, freq = ?3, datetime = ?4, rst_me = ?5, rig_me = ?6, watt_me = ?7, ant_me = ?8, qth_me = ?9, rst_counterpart = ?10, rig_counterpart = ?11, watt_counterpart = ?12, ant_counterpart = ?13, qth_counterpart = ?14, note = ?15
WHERE id = ?16
"#;
const DELETE_ELEMENT_QUERY: &str = "DELETE FROM qsl WHERE id = ?1";
const GET_QSL_PAGE_QUERY: &str = "SELECT * FROM qsl ORDER BY datetime LIMIT ?1 OFFSET ?2";
const COUNT_QUERY: &str = "SELECT COUNT(*) FROM qsl";

pub struct QSLContext {
    database: Connection,
}

impl QSLContext {
    pub fn open(db_file_path: &str) -> Result<Self, String> {
        let path_is_exist = std::path::Path::new(&db_file_path).exists();
        if !path_is_exist {
            println!("Database will be created.");
        }

        match Connection::open(db_file_path) {
            Ok(connection) => {
                if !path_is_exist {
                    println!("Enter your name (\"NOCALL\"): ");
                    let mut call_sign = String::new();
                    match std::io::stdin().read_line(&mut call_sign) {
                        Ok(_) => {
                            call_sign = call_sign.trim().parse().unwrap();
                            if call_sign.is_empty() {
                                call_sign = "NOCALL".parse().unwrap();
                            }
                            println!("Database {} will be initialized.", call_sign);
                        }
                        Err(e) => {
                            return Err(format!("Cannot read call_sign value: {}", e));
                        }
                    }

                    match connection.execute_batch(NEW_DATABASE_QUERY) {
                        Ok(_) => match connection.execute(UPDATE_SETTING_QUERY, params![call_sign])
                        {
                            Ok(_) => println!("Database is initialized, happy QSL recording!"),
                            Err(e) => {
                                return Err(format!(
                                    "Error occurred while writing callsign: {}",
                                    e
                                ));
                            }
                        },

                        Err(e) => {
                            return Err(format!(
                                "Error occurred while executing new table query: {}",
                                e
                            ));
                        }
                    }
                }
                Ok(QSLContext {
                    database: connection,
                })
            }
            Err(whatever) => Err(format!(
                "Failed to open the database with the following error: {}.",
                whatever
            )),
        }
    }

    pub fn id_is_exist(&self, id: i32) -> Result<bool, Error> {
        // Check if the entry with the given ID exists
        match self.database.prepare(CHECK_EXISTENCE_QUERY) {
            Ok(mut stmt) => match stmt.exists(params![id]) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_callsign(&self) -> Result<String, String> {
        match self
            .database
            .query_row(READ_SETTING_QUERY, [], |row| row.get(0))
        {
            Ok(call_sign) => Ok(call_sign),
            Err(e) => Err(format!("Failed to get call sign: {}", e)),
        }
    }

    pub fn add_qsl(&self, new_qsl: QSL) -> Result<(), String> {
        match &self.database.execute(
            ADD_ELEMENT_QUERY,
            params![
                &new_qsl.call_number,
                new_qsl.mode as i32,
                new_qsl.freq.as_deref(),
                &new_qsl.datetime.to_sql().unwrap(),
                new_qsl.rst_me.as_deref(),
                new_qsl.rig_me.as_deref(),
                new_qsl.watt_me,
                new_qsl.ant_me.as_deref(),
                new_qsl.qth_me.as_deref(),
                new_qsl.rst_counterpart.as_deref(),
                new_qsl.rig_counterpart.as_deref(),
                new_qsl.watt_counterpart,
                new_qsl.ant_counterpart.as_deref(),
                new_qsl.qth_counterpart.as_deref(),
                new_qsl.note.as_deref()
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn update(&self, updated_qsl: QSL) -> Result<(), String> {
        // Check if the entry with the given ID exists
        match self.id_is_exist(updated_qsl.id) {
            Ok(result) => {
                if !result {
                    return Err(format!("Id {} does not exist.", updated_qsl.id));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Could not check id {} exists. Error info {}.",
                    updated_qsl.id, e
                ));
            }
        }

        // Update the entry
        match self.database.execute(
            UPDATE_ELEMENT_QUERY,
            params![
                &updated_qsl.call_number,
                updated_qsl.mode as i32,
                updated_qsl.freq.as_deref(),
                &updated_qsl.datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                updated_qsl.rst_me.as_deref(),
                updated_qsl.rig_me.as_deref(),
                updated_qsl.watt_me,
                updated_qsl.ant_me.as_deref(),
                updated_qsl.qth_me.as_deref(),
                updated_qsl.rst_counterpart.as_deref(),
                updated_qsl.rig_counterpart.as_deref(),
                updated_qsl.watt_counterpart,
                updated_qsl.ant_counterpart.as_deref(),
                updated_qsl.qth_counterpart.as_deref(),
                updated_qsl.note.as_deref(),
                updated_qsl.id
            ],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to update QSL record: {}", e)),
        }
    }

    pub fn delete(&self, id: i32) -> Result<(), String> {
        // Check if the entry with the given ID exists
        match self.id_is_exist(id) {
            Ok(result) => {
                if !result {
                    return Err(format!("QSL with ID {} does not exist.", id));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Could not check id {} exists. Error info {}.",
                    id, e
                ));
            }
        }

        // Delete the entry
        match self.database.execute(DELETE_ELEMENT_QUERY, params![id]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete QSL record: {}", e)),
        }
    }

    fn parse_row_to_qsl(row: &Row) -> Result<QSL, Error> {
        Ok(QSL {
            id: row.get(0)?,
            call_number: row.get(1)?,
            mode: match row.get::<_, i32>(2)? {
                0 => Mode::EYEBALL,
                1 => Mode::FM,
                2 => Mode::SSB,
                3 => Mode::CW,
                4 => Mode::FTB,
                _ => Mode::OTHER,
            },
            freq: row.get(3)?,
            datetime: row.get(4)?,
            rst_me: row.get(5)?,
            rig_me: row.get(6)?,
            watt_me: row.get(7)?,
            ant_me: row.get(8)?,
            qth_me: row.get(9)?,
            rst_counterpart: row.get(10)?,
            rig_counterpart: row.get(11)?,
            watt_counterpart: row.get(12)?,
            ant_counterpart: row.get(13)?,
            qth_counterpart: row.get(14)?,
            note: row.get(15)?,
        })
    }

    pub fn get_qsl_page(&self, page_size: i64, page_number: i64) -> Result<Vec<QSL>, String> {
        let offset = page_number * page_size;
        log::debug!("Context::get_qsl_page: offset is {offset}");
        let mut stmt = self
            .database
            .prepare(GET_QSL_PAGE_QUERY)
            .map_err(|e| format!("Failed to prepare query: {}", e))?;
        let rows = stmt
            .query_map(params![page_size, offset], Self::parse_row_to_qsl)
            .map_err(|e| format!("Failed to query map: {}", e))?;

        let mut result = Vec::new();

        for row in rows {
            result.push(row.map_err(|e| format!("Failed to read row: {}", e))?);
        }
        log::debug!("Context::get_qsl_page: result's length is {}", result.len());
        Ok(result)
    }

    pub fn get_qsl_count(&self) -> Result<i64, String> {
        match self.database.query_row(COUNT_QUERY, [], |row| row.get(0)) {
            Ok(count) => Ok(count),
            Err(e) => Err(format!("Failed to get QSL count: {}", e)),
        }
    }
}
