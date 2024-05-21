// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod paths;
pub mod settings;
pub mod utils;
pub mod api;
pub mod indexing;
pub mod result;
pub mod action;