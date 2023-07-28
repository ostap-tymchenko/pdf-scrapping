use color_eyre::eyre::Result;
use pdfium_render::prelude::*;
use rusty_tesseract::{Args, Image};
use std::{
    fs::{self},
    path::Path,
};

fn pdf_to_text(pdf_path: &Path) -> String {
    let mut r = String::new();

    println!("{}", &pdf_path.display());
    Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap(),
    )
    .load_pdf_from_file(pdf_path, None)
    .unwrap()
    .pages()
    .iter()
    .for_each(|page| r.push_str(&page.text().unwrap().all()));

    r
}

fn get_name(path: &Path) -> &str {
    path.file_name()
        .expect("Couldnt find this file")
        .to_str()
        .expect("failed OsStr -> str")
}

// fn path_to_string(path: &Path) -> String {
//     path.display().to_string()
// }

fn cached_pdf_to_text(pdf_path: &Path) -> String {
    let name = get_name(pdf_path);

    // this will be the path where cached txt files will live
    let binding = "../percistant_cache/".to_owned() + name + ".txt";
    let cache_path = Path::new(&binding);

    if cache_path.exists() {
        // path_to_string(cache_path)
        // println!("read cache");
        // cache_path
        fs::read_to_string(cache_path).expect("Cant find cache")
    } else {
        // println!("created cache");
        fs::create_dir_all("../percistant_cache/").unwrap();
        let converted = pdf_to_text(pdf_path);
        fs::write(cache_path, &converted).unwrap();
        converted.clone()
    }
}

fn export_pdf_to_jpegs(path: &Path, password: Option<&str>) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable;
    // failing that, fall back to using a Pdfium library provided by the operating system.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .as_rgba8() // ... then converts it to an image::Image...
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(format!("../ocr_renders/{}.jpg", index), image::ImageFormat::Jpeg) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}

fn ocr(image_path: &Path) -> String {
    let input_image = Image::from_path(image_path).unwrap();
    let output = rusty_tesseract::image_to_string(&input_image, &Args::default()).unwrap();
    output.to_string()
}

fn pdf_to_text_ocr(pdf: &Path) -> String {
     export_pdf_to_jpegs(pdf, None);
     ocr(image_path)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let folder = Path::new("../privet/coned/");

    for pdf in fs::read_dir(folder)? {
        let pdf_path = &pdf?.path();
        let name = get_name(&pdf_path);
        let a = cached_pdf_to_text(&pdf_path);

        if a.contains("@8.8750%") {
            println!("8.875%");
        } else if a.contains("@4.5000%") {
            println!("4.475%");
        } else {
            println!("cant find %, attempting ocr");
        }
    }

    // let ind = cached_pdf_to_text(Path::new("../privet/all/Aggressive - Gas - Sep 2021.pdf")); // -> String
    // dbg!(ind);

    Ok(())
}
