// Hide console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about;
mod guide;
mod open_call;
mod workbook;

pub use open_call::OpenCall;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 768.0])
            .with_icon(
                // Optional icon
                // eframe::icon_data::from_png_bytes(&include_bytes!("../assets/win-256.png")[..])
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/mac-512.png")[..])
                    .unwrap(),
            ),

        ..Default::default()
    };

    eframe::run_native(
        "OpenCall",
        options,
        Box::new(|cc| Box::new(OpenCall::new(cc))),
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
                "app_opencall_id",
                web_options,
                Box::new(|cc| Box::new(OpenCall::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
