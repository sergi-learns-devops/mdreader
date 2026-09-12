//! Barra de herramientas superior: abrir archivo, cambiar tema y ajustar zoom.

use crate::app::{MdReaderApp, Theme};

/// Dibuja el panel superior con los controles principales.
pub fn show(app: &mut MdReaderApp, ui: &mut egui::Ui) {
    egui::Panel::top("toolbar").show(ui, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            // --- Abrir archivo ---
            if ui
                .button("📂 Abrir")
                .on_hover_text("Abrir un archivo Markdown (Ctrl+O)")
                .clicked()
            {
                app.open_dialog();
            }

            ui.separator();

            // --- Nombre del archivo abierto ---
            let title = app
                .current_file
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Ningún archivo abierto".to_owned());
            ui.label(egui::RichText::new(title).strong());

            // --- Controles a la derecha ---
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Cambio de tema.
                let (icon, tip) = match app.theme {
                    Theme::Light => ("🌙", "Cambiar a tema oscuro (Ctrl+T)"),
                    Theme::Dark => ("☀", "Cambiar a tema claro (Ctrl+T)"),
                };
                if ui.button(icon).on_hover_text(tip).clicked() {
                    app.toggle_theme();
                }

                ui.separator();

                // Controles de zoom.
                if ui.button("➕").on_hover_text("Aumentar zoom (Ctrl +)").clicked() {
                    app.zoom_in();
                }
                if ui
                    .button(format!("{}%", (app.zoom * 100.0).round() as i32))
                    .on_hover_text("Restablecer zoom (Ctrl 0)")
                    .clicked()
                {
                    app.zoom_reset();
                }
                if ui.button("➖").on_hover_text("Reducir zoom (Ctrl -)").clicked() {
                    app.zoom_out();
                }
            });
        });
        ui.add_space(4.0);
    });
}
