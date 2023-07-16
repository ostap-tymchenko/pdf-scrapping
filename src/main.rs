use rusty_tesseract::{Image, Args};
use std::process::Command;

fn image_to_text(input_image: Image) -> String {
    let output = rusty_tesseract::image_to_string(&input_image, &Args::default()).unwrap();
    output.to_string()
} 

fn generate_uniuqe_id(name: String) -> String {
    let time = SystemTime::now().duration_since(UNIX_EPOCH);
    let id = ("{time}.")

}

fn pdf_to_image(input_pdf: String) {
    let pdf_name = input_pdf.split('/').last().unwrap().split('.').nth(0).unwrap();
    let output = format!("../in_our_out/png/{pdf_name}");

    let output = Command::new("pdftoppm")
        .args(&["-png", "-r", "300", &input_pdf, &output])
        .output()
        .expect("Failed to execute pdftoppm");

    if output.status.success() {
        println!("PDF {} converted to image(/s) successfully!", pdf_name);
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Error converting PDF {pdf_name} image: {error_message}");
    }
}

fn main () {
    pdf_to_image(String::from("../in_our_out/pdf/non-client-coned-bill.pdf"));
}
