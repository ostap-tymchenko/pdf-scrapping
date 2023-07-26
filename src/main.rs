use rusty_tesseract::{Image, Args};      // ocr
use std::{fs}; 
use chrono::Local;                       // time
use std::process::Command;               // pdf to img
use rand::Rng;                           // rand

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
    fs::create_dir_all(&path).expect("cant create png dir");
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

// fn str_to_seg<'a>(segment: &'a str, substring: &'a str, split_by: char) -> &'a str {
//     segment.split(split_by).into_iter().find(|x| x.find(substring).is_some()).unwrap_or("NOT FOUND")
//     // takes a string and returns a specific substring based on a substring of the substring
//     // eg: (segment:"Sales tax: @8.875%", substring: "@", split_by: ' ') -> "@8.875"
// }

fn does_seg_have_all(segment: &str, substring:&[&str] ) -> bool {
    for s in substring.iter(){
        if !segment.find(s).is_some() {
            return false;
        } 
    } 

    true
}

enum Utility {
    Coned,
    Agressive,
    DirectEnergy,
}

enum SuplierOrUtility {
    Suplier,
    Utility,
}

enum Comodity {
    NG,
    Elec,
}

struct AccountNum(String);

struct DTHDataSetup {
    utility: Utility,
    suplier_or_utility: SuplierOrUtility,
    comodity: Comodity,
    account_num: AccountNum,
}

fn main () {
    let coned_example_bill = String::from("../in_or_out/pdf/ConedExampleBill.pdf");

    for pages in folder_image_to_text(pdf_to_image(coned_example_bill)) {
        for segment in pages.lines() {
            if does_seg_have_all(segment, &["@", "%"]) {
                // println!("{}", segment);
            }

            if does_seg_have_all(segment, &["Your account number: "]) {
                // println!("{}", segment);
            }
            println!("{segment}");
        } 
    }
}
