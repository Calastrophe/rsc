pub mod emulator;
pub mod parser;
mod file;
mod util;
mod app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rsc emulator",
        native_options,
        Box::new(|cc| Box::new(app::GUI::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "rsc_emulator", // hardcode it
                web_options,
                Box::new(|cc| Box::new(app::GUI::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}