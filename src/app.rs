//! Estado central y bucle principal de la aplicación mdReader.

use crate::ui;
use crate::watcher::FileWatcher;
use egui_commonmark::CommonMarkCache;
use std::path::PathBuf;

/// Tema visual de la aplicación.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

/// Estado global de la aplicación.
pub struct MdReaderApp {
    /// Ruta del archivo Markdown abierto actualmente, si hay alguno.
    pub current_file: Option<PathBuf>,
    /// Contenido Markdown en crudo del archivo abierto.
    pub content: String,
    /// Mensaje de error a mostrar (por ejemplo, si falla la lectura).
    pub error: Option<String>,
    /// Caché del renderizador CommonMark (imágenes, resaltado, etc.).
    pub cache: CommonMarkCache,
    /// Tema actual.
    pub theme: Theme,
    /// Factor de zoom del texto (1.0 = 100%).
    pub zoom: f32,
    /// Observador de cambios en disco para el live reload.
    pub watcher: Option<FileWatcher>,
}

/// Límites de zoom para evitar tamaños ilegibles o excesivos.
const ZOOM_MIN: f32 = 0.6;
const ZOOM_MAX: f32 = 3.0;
const ZOOM_STEP: f32 = 0.1;

impl MdReaderApp {
    /// Crea la aplicación. Si se pasó un archivo por CLI, lo carga al arrancar.
    pub fn new(cc: &eframe::CreationContext<'_>, cli_path: Option<PathBuf>) -> Self {
        // Instala los cargadores de imágenes de egui_extras para que el
        // renderizador de Markdown pueda mostrar imágenes locales.
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut app = Self {
            current_file: None,
            content: String::new(),
            error: None,
            cache: CommonMarkCache::default(),
            theme: Theme::Light,
            zoom: 1.0,
            watcher: None,
        };

        if let Some(path) = cli_path {
            app.load_file(path);
        }

        app
    }

    /// Abre el diálogo nativo de selección de archivos.
    pub fn open_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "mdown", "mkd", "txt"])
            .set_title("Abrir archivo Markdown")
            .pick_file()
        {
            self.load_file(path);
        }
    }

    /// Carga un archivo desde disco en el estado de la aplicación y arranca
    /// el watcher para el live reload.
    pub fn load_file(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                self.content = text;
                self.error = None;
                // Arranca (o reinicia) el observador de cambios sobre el archivo.
                self.watcher = FileWatcher::new(&path).ok();
                self.current_file = Some(path);
            }
            Err(e) => {
                self.error = Some(format!("No se pudo abrir «{}»: {}", path.display(), e));
            }
        }
    }

    /// Recarga el contenido del archivo actualmente abierto desde disco.
    fn reload_current(&mut self) {
        if let Some(path) = self.current_file.clone() {
            if let Ok(text) = std::fs::read_to_string(&path) {
                self.content = text;
                self.error = None;
            }
        }
    }

    /// Aumenta el zoom del texto dentro de los límites permitidos.
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + ZOOM_STEP).min(ZOOM_MAX);
    }

    /// Reduce el zoom del texto dentro de los límites permitidos.
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - ZOOM_STEP).max(ZOOM_MIN);
    }

    /// Restablece el zoom al 100 %.
    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
    }

    /// Alterna entre tema claro y oscuro.
    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        };
    }

    /// Procesa los archivos soltados sobre la ventana (drag & drop).
    fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        // Recogemos la ruta del primer archivo soltado (si lo hay) dentro del
        // closure de input, para no mantener prestado el contexto al cargar.
        let dropped_path = ctx.input(|i| {
            i.raw
                .dropped_files
                .first()
                .map(|f| f.path().to_path_buf())
        });
        if let Some(path) = dropped_path {
            self.load_file(path);
        }
    }

    /// Lee los atajos de teclado del frame actual y ejecuta la acción asociada.
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Extraemos las intenciones dentro del closure de input y actuamos fuera,
        // para no mantener prestado el contexto mientras mutamos el estado.
        let (open, zoom_in, zoom_out, zoom_reset, toggle_theme) = ctx.input(|i| {
            let cmd = i.modifiers.command;
            (
                cmd && i.key_pressed(egui::Key::O),
                cmd && (i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals)),
                cmd && i.key_pressed(egui::Key::Minus),
                cmd && i.key_pressed(egui::Key::Num0),
                cmd && i.key_pressed(egui::Key::T),
            )
        });

        if open {
            self.open_dialog();
        }
        if zoom_in {
            self.zoom_in();
        }
        if zoom_out {
            self.zoom_out();
        }
        if zoom_reset {
            self.zoom_reset();
        }
        if toggle_theme {
            self.toggle_theme();
        }
    }

    /// Comprueba el watcher y recarga el documento si el archivo cambió.
    fn handle_live_reload(&mut self, ctx: &egui::Context) {
        let changed = self.watcher.as_ref().map(|w| w.changed()).unwrap_or(false);
        if changed {
            // Pequeña espera implícita: algunos editores guardan en dos pasos.
            self.reload_current();
            ctx.request_repaint();
        }
    }
}

impl eframe::App for MdReaderApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Aplica el tema visual seleccionado.
        match self.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
        }

        // Aplica el factor de zoom a toda la interfaz.
        ctx.set_zoom_factor(self.zoom);

        // Procesa entrada del usuario y eventos del sistema.
        self.handle_shortcuts(&ctx);
        self.handle_dropped_files(&ctx);
        self.handle_live_reload(&ctx);

        // Barra de herramientas superior.
        ui::toolbar::show(self, ui);

        // Área principal de lectura.
        ui::viewer::show(self, ui);

        // Repintado periódico para que el live reload responda con fluidez
        // aunque no haya interacción del usuario.
        ctx.request_repaint_after(std::time::Duration::from_millis(300));
    }
}
