# mdReader

Un lector de Markdown **ligero y nativo** para leer archivos `README.md` y otros documentos Markdown de forma cómoda, sin necesidad de abrir un IDE ni un navegador.

Construido en **Rust** con [`egui`](https://github.com/emilk/egui), usa su propio motor de renderizado (vía GPU) en lugar de un WebView. Esto significa: **sin Chromium, sin Electron, sin dependencias de runtime** y un binario autónomo de ~9 MB.

## Características (MVP)

- 📂 **Abrir archivos** de varias formas:
  - Diálogo nativo (`Ctrl+O`)
  - Arrastrar y soltar (drag & drop) sobre la ventana
  - Como argumento en la línea de comandos: `mdreader README.md`
- 📝 **Renderizado de Markdown** (CommonMark / GFM): encabezados, listas, tablas, bloques de código con resaltado de sintaxis, citas, enlaces e imágenes.
- 🌗 **Tema claro / oscuro** conmutable (`Ctrl+T`).
- 🔍 **Zoom de texto** ajustable (`Ctrl` `+` / `-`, restablecer con `Ctrl+0`).
- 🔄 **Recarga en vivo** (live reload): el documento se actualiza automáticamente cuando el archivo cambia en disco.

## Atajos de teclado

| Acción              | Atajo      |
|---------------------|------------|
| Abrir archivo       | `Ctrl + O` |
| Aumentar zoom       | `Ctrl + +` |
| Reducir zoom        | `Ctrl + -` |
| Restablecer zoom    | `Ctrl + 0` |
| Cambiar tema        | `Ctrl + T` |

## Compilación

### Requisitos

- [Rust](https://www.rust-lang.org/) (edición 2021 o superior).
- En Windows con el toolchain GNU, se necesita **MinGW-w64** en el `PATH` (proporciona `gcc` y `dlltool`). Con el toolchain MSVC, se necesitan las *Build Tools de Visual Studio* con el componente de C++.

### Pasos

```bash
# Clonar el repositorio
git clone https://github.com/<usuario>/mdReader.git
cd mdReader

# Compilar y ejecutar en modo desarrollo
cargo run -- README.md

# Compilar el binario optimizado de release
cargo build --release
# El ejecutable queda en target/release/mdreader.exe (Windows)
```

## Uso

- Ejecuta `mdreader` y abre un archivo con `Ctrl+O`, o
- Arrastra un `.md` sobre la ventana, o
- Pásalo directamente: `mdreader ruta/al/archivo.md`

## Arquitectura

```
src/
├── main.rs          Punto de entrada; configura la ventana y procesa el argumento CLI.
├── app.rs           Estado central y bucle principal (tema, zoom, live reload, drag&drop, atajos).
├── watcher.rs       Observador del sistema de ficheros (notify) para la recarga en vivo.
└── ui/
    ├── toolbar.rs   Barra superior: abrir, tema, zoom.
    └── viewer.rs    Área de lectura: renderiza el Markdown en una columna centrada.
```

### Dependencias principales

| Crate             | Uso                                             |
|-------------------|-------------------------------------------------|
| `eframe` / `egui` | Framework de GUI nativo (motor propio).         |
| `egui_commonmark` | Renderizado de CommonMark/GFM con resaltado.    |
| `rfd`             | Diálogo de apertura de archivos nativo.         |
| `notify`          | Watcher del sistema de ficheros (live reload).  |

## Estado del proyecto

MVP funcional para **Windows**. El diseño es multiplataforma (Windows, Linux, macOS) gracias a `egui`; el soporte de compilación y empaquetado para Linux y macOS está planificado.

## Hoja de ruta

- [ ] Panel lateral con tabla de contenidos (TOC) navegable.
- [ ] Búsqueda dentro del documento (`Ctrl+F`).
- [ ] Explorador de archivos de la carpeta.
- [ ] Compilación y empaquetado para Linux y macOS.
- [ ] Asociación de archivos `.md` al sistema.
- [ ] Exportar a HTML / PDF.

## Licencia

MIT.
