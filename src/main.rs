use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use winapi::um::minwinbase::SYSTEMTIME;
use winapi::um::sysinfoapi::SetLocalTime;

const URL_DATE_TIME: &str = "http://worldtimeapi.org/api/timezone/America/Sao_Paulo";

#[derive(Serialize, Deserialize, Debug)]
struct DateTimeApiReturn {
    datetime: String,
    day_of_week: u8,
    day_of_year: u32,
    timezone: String,
    unixtime: i64,
    utc_datetime: String,
    utc_offset: String,
    week_number: u8,
}

#[tokio::main]
async fn main() {
    let request_datetime = get_current_datetime().await;

    if request_datetime.is_err() {
        handle_error_logs(&request_datetime.as_ref().unwrap_err());
    }

    let payload_datetime = request_datetime.unwrap();
    let separete_domain: Vec<&str> = payload_datetime.datetime.split("T").collect();

    let date: Vec<&str> = separete_domain.get(0).unwrap().split("-").collect();
    let time: Vec<&str> = separete_domain.get(1).unwrap().split(":").collect();

    let year = date.get(0).unwrap();
    let month = date.get(1).unwrap();
    let day = date.get(2).unwrap();

    let hour = time.get(0).unwrap();
    let minute = time.get(1).unwrap();

    let separeted: Vec<&str> = time.get(2).unwrap().split(".").collect();
    let separeted_tow: Vec<&str> = separeted.get(1).unwrap().split("-").collect();

    let seconds = separeted.get(0).unwrap();
    let mileseconds = separeted_tow.get(0).unwrap();

    let mut current_payload = SYSTEMTIME {
        wDay: day.parse::<u16>().unwrap(),
        wYear: year.parse::<u16>().unwrap(),
        wMonth: month.parse::<u16>().unwrap(),
        wDayOfWeek: payload_datetime.day_of_week as u16,
        wHour: hour.parse::<u16>().unwrap(),
        wMinute: minute.parse::<u16>().unwrap(),
        wSecond: seconds.parse::<u16>().unwrap(),
        wMilliseconds: mileseconds[0..2].parse::<u32>().unwrap() as u16,
    };

    unsafe {
        if SetLocalTime(&mut current_payload) == 0 {
            handle_error_logs(&format!("Error in execution: {:?}", Error::last_os_error()));
        }
    }
}

async fn get_current_datetime() -> Result<DateTimeApiReturn, String> {
    let response = reqwest::get(URL_DATE_TIME).await;

    if response.is_err() {
        return Err("Error in request endpoint.".to_owned());
    }

    let response_data = response
        .unwrap()
        .text()
        .await
        .expect("Error in conversion to text");
    let payload_data: DateTimeApiReturn = serde_json::from_str(response_data.as_str()).unwrap();

    Ok(payload_data)
}

fn handle_error_logs(err: &str) {
    let logs_file_path = Path::new("./logs_error.txt");
    let seconds = Local::now().second();

    match File::open(logs_file_path) {
        Ok(mut file) => {
            let mut content_buffer = String::new();
            let _ = file.read_to_string(&mut content_buffer);
            content_buffer.push_str(&format!("{} - [{}]\n", err, seconds));

            if let Err(e) = File::create(logs_file_path)
                .and_then(|mut nfile| nfile.write_all(content_buffer.as_bytes()))
            {
                eprintln!("Error writing to file: {}", e);
            }
        }
        Err(_) => {
            if let Err(e) = File::create(logs_file_path).and_then(|mut file| {
                file.write_all(format!("{} - [{}]\n", err, seconds).as_bytes())
            }) {
                eprintln!("Error creating file: {}", e);
            }
        }
    }
}
