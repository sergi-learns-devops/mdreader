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
            .with_icon(load_icon())
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "mdReader",
        native_options,
        Box::new(move |cc| Ok(Box::new(MdReaderApp::new(cc, cli_path.clone())))),
    )
}

/// Decodifica el icono de la aplicación (embebido en el binario) a un
/// [`egui::IconData`] para usarlo como icono de la ventana y de la barra de
/// tareas. Si la decodificación fallara, devuelve un icono vacío.
fn load_icon() -> egui::IconData {
    // El PNG se incrusta en el binario en tiempo de compilación.
    const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");

    match image::load_from_memory(ICON_PNG) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            egui::IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            }
        }
        Err(_) => egui::IconData::default(),
    }
}
