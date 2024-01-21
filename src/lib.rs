// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
pub mod api;
pub mod dialog;
pub mod extensions;
pub mod results;
pub mod paths;
pub mod utils;
pub mod settings;
