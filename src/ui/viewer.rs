//! Área principal de lectura: renderiza el Markdown, la pantalla de bienvenida
//! y los mensajes de error.

use crate::app::MdReaderApp;
use egui_commonmark::CommonMarkViewer;

/// Ancho máximo de la columna de lectura (antes de aplicar zoom), en píxeles.
/// Un ancho acotado mejora la legibilidad en textos largos.
const READING_WIDTH: f32 = 720.0;

/// Dibuja el panel central con el contenido del documento.
pub fn show(app: &mut MdReaderApp, ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |ui| {
        // Mensaje de error, si lo hay.
        if let Some(err) = &app.error {
            ui.colored_label(egui::Color32::from_rgb(200, 60, 60), err);
            ui.separator();
        }

        // Sin archivo abierto: pantalla de bienvenida.
        if app.current_file.is_none() {
            welcome_screen(ui);
            return;
        }

        // Documento renderizado dentro de una columna de lectura centrada.
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let available = ui.available_width();
                let side_margin = ((available - READING_WIDTH) / 2.0).max(0.0);

                ui.horizontal(|ui| {
                    ui.add_space(side_margin);
                    ui.vertical(|ui| {
                        ui.set_max_width(READING_WIDTH);
                        CommonMarkViewer::new().show(ui, &mut app.cache, &app.content);
                    });
                });
            });
    });
}

/// Pantalla mostrada cuando no hay ningún documento abierto.
fn welcome_screen(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(120.0);
        ui.heading("mdReader");
        ui.add_space(8.0);
        ui.label("Lector de Markdown ligero y nativo.");
        ui.add_space(24.0);
        ui.label("Para empezar:");
        ui.add_space(8.0);
        ui.label("• Pulsa «📂 Abrir» o Ctrl+O");
        ui.label("• Arrastra un archivo .md sobre esta ventana");
        ui.label("• Abre un .md pasándolo como argumento en la terminal");
    });
}
