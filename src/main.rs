use log::warn;
use maurice_lib::gui::Gui;
use maurice_lib::hardware::k7::K7;
#[cfg(not(target_family = "wasm"))]
use {
    clap::Parser,
    maurice_lib::args::Args,
    maurice_lib::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH},
};

#[cfg(not(target_family = "wasm"))]
fn main() {
    env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_drag_and_drop(true)
            .with_inner_size([
                (DEFAULT_PIXEL_SIZE * WIDTH) as f32,
                (DEFAULT_PIXEL_SIZE * HEIGHT) as f32,
            ]),
        ..Default::default()
    };
    let args = Args::parse();
    let mut gui = Gui::default();
    if let Some(k7_file) = args.k7 {
        match K7::try_from(k7_file) {
            Ok(k7) => gui.set_k7(k7),
            Err(e) => warn!("Unable to open tape {e}"),
        }
    }
    let _ = eframe::run_native(
        "Maurice",
        native_options,
        Box::new(|_cc| Ok(Box::default())),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");
        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::<Gui>::default())),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
