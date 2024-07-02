// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::anyhow;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chacha20poly1305::aead::{stream, NewAead};
use chacha20poly1305::XChaCha20Poly1305;
use cipher_crypt::{Caesar, Cipher};
use log::info;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Bytes, Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_endpoints, log_message])
        //.invoke_handler(tauri::generate_handler![log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn test_endpoints() -> bool {
    fn check_web_server(url: &str) -> Result<(),()> {
        let response = reqwest::blocking::get(url);

        // Check if the request was successful, ignore the response status 
        if let Ok(response_ok) = response{
            return Ok(());
        }

        return Err(());
    }
    let bool_test1;
    let bool_test2;
    match check_web_server("http://127.0.0.1:5001") {
        Ok(_) => {
            //println!("RPC is up");
     bool_test1 = true;
        }
        Err(_) => {
            //println!("Error: {}", e);:#![warn()]
            bool_test1 = false;
        }
    }
    match check_web_server("http://127.0.0.1:8080") {
        Ok(_) => {
            //println!("gateway is up");
            bool_test2 = true;
        }
        Err(_) => {
            //println!("Error: {}", e);
            bool_test2 = false;
        }
    }
    if bool_test2 && bool_test1 {
        //typewrite("All systems operational.");
        println!("true");
        return true;
    }
    println!("false");
    return false;
}
#[tauri::command]
fn log_message(message: String){
    info!("{}", message);
}
