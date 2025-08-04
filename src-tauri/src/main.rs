// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
}

#[cfg(debug_assertions)]
pub fn export_types() {
    use ts_rs::TS;

    app_lib::stages::ProcessingStage::export().unwrap();
}
