use arboard::Clipboard;
use opener;
use qrcode_generator::QrCodeEcc;
use std::io::Write;

fn main() {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    // Read string from clipboard
    if let Ok(str) = clipboard.get_text() {
        // Get path of the temp dir
        let mut path = std::env::temp_dir();
        path.push("qrcode.png");
        println!("Path: {}", path.display());
        qrcode_generator::to_png_to_file(str, QrCodeEcc::Low, 1024, &path)
            .expect("Failed to generate QR code");
        opener::open(path).expect("Failed to open QR code");
        return;
    }
    writeln!(std::io::stderr(), "Failed to read text from clipboard").unwrap();
    std::process::exit(1);
}
