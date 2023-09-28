#![warn(clippy::all, clippy::nursery, clippy::cargo)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_family = "wasm"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    #[cfg(target_family = "windows")]
    let native_options = eframe::NativeOptions {
        icon_data: load_favicon(),
        ..Default::default()
    };

    #[cfg(not(target_family = "windows"))]
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "SModelHexer",
        native_options,
        Box::new(|cc| Box::new(SModelHexer::app::SModelHexerApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_family = "wasm")]
fn main() {
    use poll_promise::Promise;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let _ = Promise::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(SModelHexer::app::SModelHexerApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[cfg(target_family = "windows")]
fn load_favicon() -> Option<eframe::IconData> {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/icon-256.png");
        let image = match image::load_from_memory(icon) {
            Ok(i) => i.into_rgba8(),
            Err(_) => return None,
        };
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Some(eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}
