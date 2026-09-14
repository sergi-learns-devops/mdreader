//! Área principal de lectura: renderiza el Markdown, la pantalla de bienvenida,
//! los mensajes de error y el overlay de arrastrar y soltar.

use crate::app::MdReaderApp;
use egui_commonmark::CommonMarkViewer;

/// Ancho máximo de la columna de lectura (antes de aplicar zoom), en píxeles.
/// Un ancho acotado mejora la legibilidad en textos largos.
const READING_WIDTH: f32 = 720.0;

/// Dibuja el panel central con el contenido del documento.
pub fn show(app: &mut MdReaderApp, ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |ui| {
        // Mensaje de error, si lo hay, en una tarjeta destacada.
        if let Some(err) = app.error.clone() {
            error_card(ui, &err);
        }

        // Sin archivo abierto (y sin error): pantalla de bienvenida.
        if app.current_file.is_none() && app.error.is_none() {
            welcome_screen(ui);
        } else if app.current_file.is_some() {
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
                            // CommonMarkViewer renderiza también los checkboxes GFM
                            // (`- [ ]` / `- [x]`) presentes en muchos README.
                            // enable_scroll_to_heading permite que el TOC haga
                            // scroll a los encabezados (que llevan `{#slug}`).
                            CommonMarkViewer::new()
                                .enable_scroll_to_heading(true)
                                .show(ui, &mut app.cache, &app.rendered_content);
                        });
                    });
                });
        }
    });

    // Overlay de arrastrar y soltar: se muestra encima de todo mientras el
    // usuario arrastra uno o más archivos sobre la ventana.
    draw_drag_and_drop_overlay(ui.ctx());
}

/// Muestra un mensaje de error dentro de una tarjeta con color de aviso.
fn error_card(ui: &mut egui::Ui, message: &str) {
    egui::Frame::group(ui.style())
        .fill(ui.visuals().extreme_bg_color)
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 80, 80)))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚠").color(egui::Color32::from_rgb(200, 80, 80)));
                ui.label(egui::RichText::new(message).color(egui::Color32::from_rgb(200, 80, 80)));
            });
        });
    ui.add_space(6.0);
}

/// Pantalla mostrada cuando no hay ningún documento abierto.
fn welcome_screen(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        ui.heading("mdReader");
        ui.add_space(8.0);
        ui.label("Lector de Markdown ligero y nativo.");
        ui.add_space(28.0);
        ui.label(egui::RichText::new("Para empezar").strong());
        ui.add_space(10.0);
        ui.label("📂  Pulsa «Abrir» o Ctrl+O");
        ui.label("🖱  Arrastra un archivo .md sobre esta ventana");
        ui.label("⌨  Ábrelo desde la terminal:  mdreader archivo.md");
        ui.add_space(20.0);
        ui.label(
            egui::RichText::new("F5 recarga · Ctrl+T tema · Ctrl +/- zoom")
                .weak()
                .small(),
        );
    });
}

/// Dibuja un overlay semitransparente sobre toda la ventana mientras se
/// arrastran archivos, indicando visualmente que se pueden soltar.
fn draw_drag_and_drop_overlay(ctx: &egui::Context) {
    let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
    if !hovering {
        return;
    }

    let screen_rect = ctx.content_rect();
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("dnd_overlay"),
    ));

    // Velo semitransparente.
    painter.rect_filled(
        screen_rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(20, 20, 24, 180),
    );

    // Texto centrado con la instrucción.
    painter.text(
        screen_rect.center(),
        egui::Align2::CENTER_CENTER,
        "Suelta aquí para abrir el archivo",
        egui::FontId::proportional(28.0),
        egui::Color32::WHITE,
    );
}
