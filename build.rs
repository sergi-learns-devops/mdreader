//! Script de compilación.
//!
//! En Windows, embebe el icono de la aplicación (`assets/icon.ico`) en el
//! ejecutable para que se muestre en el Explorador de archivos, la barra de
//! tareas y el diálogo de propiedades. En otras plataformas no hace nada.

fn main() {
    #[cfg(windows)]
    {
        // Recompila si el icono cambia.
        println!("cargo:rerun-if-changed=assets/icon.ico");

        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        if let Err(e) = res.compile() {
            // No abortamos la compilación por un fallo al embeber el icono;
            // solo avisamos, para que la app siga compilando sin él.
            println!("cargo:warning=No se pudo embeber el icono: {e}");
        }
    }
}
