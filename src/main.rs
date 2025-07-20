mod qsl_context;
mod qsl_manager;
mod qsl_type;
mod qsl_ui;

use crate::qsl_context::Context;
use crate::qsl_manager::QSLManager;
use crate::qsl_ui::{edit_record_dialog, show_qsl_table};
use cursive::reexports::log::LevelFilter;
use cursive::views::Dialog;
use cursive::{logger, menu};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: qsl <sqlite file path> [--html]");
        return;
    }
    let db_file_path = args[2].to_string();

    match Context::open(&db_file_path) {
        Ok(context) => {
            let mut siv = cursive::default();
            logger::set_internal_filter_level(LevelFilter::Warn);
            logger::set_external_filter_level(LevelFilter::Debug);
            logger::init();

            // Menubar
            siv.menubar()
                // We add a new "File" tree
                .add_subtree(
                    "Record",
                    menu::Tree::new()
                        // Trees are made of leaves, with are directly actionable...
                        .leaf("New", move |s| {
                            edit_record_dialog(s, None);
                        })
                        .leaf("Output to html", move |s| {
                            s.add_layer(Dialog::info("Coming soon..."))
                        }),
                )
                .add_subtree(
                    "Help",
                    menu::Tree::new()
                        .leaf("Log", |s| s.toggle_debug_console())
                        .leaf("About", |s| s.add_layer(Dialog::info("qsl_record 0.1.0"))),
                )
                .add_delimiter()
                .add_leaf("Quit", |s| {
                    s.add_layer(
                        Dialog::text("Are you sure?")
                            .title("Confirm")
                            .button("Yes", |s| {
                                s.quit();
                            }),
                    )
                });
            siv.set_autohide_menu(false);

            match QSLManager::new(context) {
                Ok(context) => siv.set_user_data(context),
                Err(e) => {
                    eprintln!("Failed to read the callsign!\nDetail: {}", e);
                    return;
                }
            }

            show_qsl_table(&mut siv);

            siv.run();
        }
        Err(e) => eprintln!("Failed to initialize the context!\nDetail: {}", e),
    }
}
