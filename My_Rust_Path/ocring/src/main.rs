use tesseract_rs::{Tesseract, PageSegMode};

fn main() {
    let mut tess = Tesseract::new();
    tess.set_lang("hin").unwrap(); // Set language to Hindi
    tess.set_page_seg_mode(PageSegMode::PSM_AUTO); // Set page segmentation mode to automatic

    let image_path = "path/to/your/image.png";
    let ocr_text = tess.get_text_from_file(image_path).unwrap();

    println!("{}", ocr_text);
}
