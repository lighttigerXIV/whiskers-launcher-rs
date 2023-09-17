// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
pub mod results;
pub mod extensions;
pub mod paths;
pub mod settings;
pub mod others;