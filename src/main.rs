fn main() {
    if let Err(err) = pdf::run() {
        if let Some(pdf_err) = err.downcast_ref::<pdf::core::PdfError>() {
            eprintln!("error[{}]: {}", pdf_err.code(), pdf_err);
        } else {
            eprintln!("error[internal]: {err}");
        }
        std::process::exit(1);
    }
}
