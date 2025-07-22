mod qsl_context;
mod qsl_manage_ui;
mod qsl_manager;
mod qsl_template;
mod qsl_type;

use crate::qsl_context::QSLContext;
use crate::qsl_manage_ui::{edit_record_dialog, show_qsl_table};
use crate::qsl_manager::QSLManager;
use crate::qsl_type::Usage;
use cursive::event::{Event, Key};
use cursive::reexports::log;
use cursive::reexports::log::LevelFilter;
use cursive::views::Dialog;
use cursive::{logger, menu};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: qsl <sqlite file path> [--html <path> | --typst <path>]");
        return;
    }
    let db_file_path = args[1].to_string();
    let mode = if args.len() > 2 {
        let arg = args[2].to_string();
        if arg == "--html" {
            Usage::HTML
        } else if arg == "--typst" {
            Usage::TYPST
        } else {
            Usage::UI
        }
    } else {
        Usage::UI
    };
    let output_path = if mode == Usage::HTML || mode == Usage::TYPST {
        if args.len() < 4 {
            eprintln!("Require a path");
            return;
        }
        args[3].to_string()
    } else {
        "".to_string()
    };

    match QSLContext::open(&db_file_path) {
        Ok(context) => {
            let qsl_manager = match QSLManager::new(context, 18) {
                Ok(context) => context,
                Err(e) => {
                    eprintln!("Failed to read the callsign!\nDetail: {}", e);
                    return;
                }
            };

            match mode {
                Usage::HTML => {
                    let path = Path::new(&output_path);
                    if !path.exists() {
                        eprintln!("Folder not exists.");
                        return;
                    }
                    if !path.is_dir() {
                        eprintln!("The provided path is not a directory.");
                        return;
                    }
                    if !path.read_dir().unwrap().next().is_none() {
                        eprintln!("The provided directory is not empty.");
                        return;
                    }
                    match qsl_manager.output_html(path) {
                        Ok(()) => {
                            println!("Successful outputting html.");
                            return;
                        }
                        Err(err) => {
                            eprintln!("Failed to outputting html: {err}");
                            return;
                        }
                    }
                }
                Usage::TYPST => match File::create_new(output_path) {
                    Ok(mut file) => match qsl_manager.output_typst(&mut file) {
                        Ok(()) => {
                            println!("Successful writing to file.");
                            return;
                        }
                        Err(err) => {
                            eprintln!("Failed to write to the file: {err}");
                            return;
                        }
                    },

                    Err(err) => {
                        eprintln!("Failed to create the file: {err}");
                        return;
                    }
                },
                Usage::UI => {
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
                                .leaf("About", |s| {
                                    s.add_layer(Dialog::info(
                                        "qsl_record 0.1.0\nby BenderBlog Rodriguez, 2025-07-20",
                                    ))
                                }),
                        )
                        .add_delimiter()
                        .add_leaf("Quit", |s| {
                            s.add_layer(
                                Dialog::text("Are you sure you want to quit?")
                                    .button("No", |s| {
                                        s.pop_layer();
                                    })
                                    .button("Yes", |s| {
                                        s.quit();
                                    }),
                            )
                        });
                    siv.set_autohide_menu(false);

                    siv.set_user_data(qsl_manager);

                    siv.add_global_callback('n', |s| {
                        edit_record_dialog(s, None);
                    });

                    siv.add_global_callback(Event::Key(Key::Esc), |s| {
                        log::debug!("s.screen.len {}", s.screen().len());
                        if s.screen().len() > 1 {
                            s.pop_layer();
                        } else {
                            s.add_layer(
                                Dialog::text("Are you sure you want to quit?")
                                    .title("Confirm")
                                    .button("No", |s| {
                                        s.pop_layer();
                                    })
                                    .button("Yes", |s| {
                                        s.quit();
                                    }),
                            );
                        }
                    });

                    show_qsl_table(&mut siv);

                    siv.run();
                }
            }

            if mode == Usage::HTML || mode == Usage::TYPST {
            } else {
            }
        }
        Err(e) => eprintln!("Failed to initialize the context!\nDetail: {}", e),
    }
}
