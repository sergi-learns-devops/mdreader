// Evita que se abra la consola de Windows al ejecutar la app en modo release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;
mod watcher;

use app::MdReaderApp;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    // Permite abrir un archivo pasándolo como argumento en la línea de comandos
    // (por ejemplo, al asociar los .md a la app o arrastrarlos sobre el .exe).
    let cli_path: Option<PathBuf> = std::env::args().nth(1).map(PathBuf::from);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 750.0])
            .with_min_inner_size([480.0, 360.0])
            .with_title("mdReader")
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "mdReader",
        native_options,
        Box::new(move |cc| Ok(Box::new(MdReaderApp::new(cc, cli_path.clone())))),
    )
}
