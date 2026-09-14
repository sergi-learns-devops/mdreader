//! Panel lateral con la tabla de contenidos (TOC).
//!
//! Muestra la lista de encabezados del documento, indentados según su nivel.
//! Al hacer clic en uno, solicita al renderizador que haga scroll hasta él.

use crate::app::MdReaderApp;

/// Ancho del panel lateral de la tabla de contenidos.
const TOC_WIDTH: f32 = 260.0;

/// Dibuja el panel lateral del TOC si está activado y hay encabezados.
pub fn show(app: &mut MdReaderApp, ui: &mut egui::Ui) {
    if !app.show_toc || app.headings.is_empty() {
        return;
    }

    // Recogemos el ancla a la que hay que hacer scroll tras dibujar la lista,
    // para no mutar el estado mientras iteramos sobre los encabezados.
    let mut scroll_target: Option<String> = None;

    egui::Panel::left("toc_panel")
        .resizable(true)
        .default_size(TOC_WIDTH)
        .show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(6.0);
                ui.label(egui::RichText::new("Contenido").strong());
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for heading in &app.headings {
                        // Indentación proporcional al nivel del encabezado.
                        let indent = (heading.level.saturating_sub(1)) as f32 * 14.0;
                        ui.horizontal(|ui| {
                            ui.add_space(6.0 + indent);
                            // Los encabezados de nivel 1 se muestran destacados.
                            let text = if heading.level == 1 {
                                egui::RichText::new(&heading.text).strong()
                            } else {
                                egui::RichText::new(&heading.text)
                            };
                            if ui
                                .add(egui::Label::new(text).sense(egui::Sense::click()))
                                .on_hover_text("Ir a esta sección")
                                .clicked()
                            {
                                scroll_target = Some(heading.slug.clone());
                            }
                        });
                        ui.add_space(2.0);
                    }
                });
        });

    if let Some(slug) = scroll_target {
        app.scroll_to_heading(&slug);
    }
}
