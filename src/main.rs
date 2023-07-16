use rusty_tesseract::{Image, Args};      // ocr
use std::{time::{SystemTime, UNIX_EPOCH}, fs}; // time
use chrono::Local;                       // time
use std::process::Command;               // pdf to img
use rand::Rng;                           // rand
use std::path::Path;                     // checking if path exists

fn image_to_text(image_path: String) -> String {
    dbg!(&image_path);
    let input_image = Image::from_path(image_path).unwrap();
    let output = rusty_tesseract::image_to_string(&input_image, &Args::default()).unwrap();
    output.to_string()
}

fn folder_image_to_text(folder_path: String) -> Vec<String> {
    let dir = fs::read_dir(folder_path).unwrap();
    let mut output: Vec<String> = Vec::new();

    for image in dir {
        // dbg!(image);
        output.push(image_to_text(image.unwrap().path().into_os_string().into_string().unwrap()))
    }

    output
}

fn generate_id(name: &str) -> String {
    // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let date = Local::now();
    let timestamp = format!("{}", date.format("%Y-%m-%d-%H:%M:%S"));

    let random = rand::thread_rng().gen_range(10000..999999999); 
    let id = String::from(format!("{}-rand-{}-{}", timestamp, random, name));

    id
}

fn pdf_to_image(input_pdf: String) -> String {
    let pdf_name = input_pdf.split('/').last().unwrap().split('.').nth(0).unwrap();
    let id = generate_id(pdf_name);

    let path = format!("../in_or_out/png/{id}");
    fs::create_dir_all(&path);
    let output = path.clone() + "/" + pdf_name;
    // let output = format!("../in_or_out/png/{id}/{pdf_name}");

    let run_command_output = Command::new("pdftoppm")
        .args(&["-png", "-r", "300", &input_pdf, &output])
        .output()
        .expect("Failed to execute pdftoppm");

    if run_command_output.status.success() {
        println!("PDF {} converted to image(/s) successfully!", pdf_name);
    } else {
        let error_message = String::from_utf8_lossy(&run_command_output.stderr);
        println!("Error converting PDF {pdf_name} image: {error_message}");
    }

    path
}

fn main () {
    // pdf_to_image(String::from("../in_or_out/pdf/non-client-coned-bill.pdf"));
    let test_input_pdf_1 = String::from("../in_or_out/pdf/non-client-coned-bill.pdf");
    let a = folder_image_to_text(pdf_to_image(test_input_pdf_1));
    dbg!(a);
}
