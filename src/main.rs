use tesseract::Tesseract;

fn main() {
    let mut tess = Tesseract::new();
    tess.set_rectangle(430, 230, 180, 60);
}
