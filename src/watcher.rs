//! Observador del sistema de ficheros para recargar el documento
//! automáticamente cuando el archivo abierto cambia en disco (live reload).

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};

/// Encapsula el watcher de `notify` y el canal por el que llegan los eventos
/// de modificación. Mientras esta estructura viva, el archivo se vigila.
pub struct FileWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<()>,
}

impl FileWatcher {
    /// Crea un watcher que vigila el `path` indicado. Cada vez que el archivo
    /// se modifica, se envía una señal por el canal interno.
    pub fn new(path: &Path) -> notify::Result<Self> {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                use notify::EventKind;
                // Nos interesan modificaciones y creaciones (algunos editores
                // reescriben el archivo completo al guardar).
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    // Ignoramos el error si el receptor ya no existe.
                    let _ = tx.send(());
                }
            }
        })?;

        // Vigilamos el archivo concreto. En algunos sistemas conviene vigilar
        // el directorio contenedor, pero para un único archivo esto es suficiente.
        watcher.watch(path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    /// Devuelve `true` si el archivo ha cambiado desde la última comprobación.
    /// Drena todos los eventos pendientes para no recargar varias veces seguidas.
    pub fn changed(&self) -> bool {
        let mut changed = false;
        while self.rx.try_recv().is_ok() {
            changed = true;
        }
        changed
    }
}
