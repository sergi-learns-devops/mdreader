//! Tabla de contenidos (TOC): extracción de encabezados del Markdown,
//! generación de anclas (slugs) y preparación del texto para que el
//! renderizador pueda hacer scroll a cada encabezado.
//!
//! egui_commonmark solo hace scroll a encabezados que tengan un id EXPLÍCITO
//! con la sintaxis `# Título {#id}`. Como los README normales no incluyen esa
//! sintaxis, inyectamos nosotros un `{#slug}` al final de cada línea de
//! encabezado. Así la navegación funciona con cualquier documento.

/// Un encabezado del documento para mostrar en la tabla de contenidos.
#[derive(Clone)]
pub struct Heading {
    /// Nivel del encabezado (1 = `#`, 2 = `##`, …, 6 = `######`).
    pub level: usize,
    /// Texto visible del encabezado (sin las almohadillas ni el `{#id}`).
    pub text: String,
    /// Ancla única usada para el scroll (coincide con el id inyectado).
    pub slug: String,
}

/// Resultado de procesar el Markdown: el texto listo para renderizar (con los
/// `{#slug}` inyectados) y la lista de encabezados extraídos.
pub struct TocResult {
    pub rendered_markdown: String,
    pub headings: Vec<Heading>,
}

/// Procesa el contenido Markdown: extrae los encabezados e inyecta un id
/// (`{#slug}`) en cada línea de encabezado que no lo tenga ya.
///
/// Respeta los bloques de código delimitados por vallas (``` o ~~~) para no
/// confundir líneas que empiezan por `#` dentro del código con encabezados.
pub fn process(markdown: &str) -> TocResult {
    let mut headings: Vec<Heading> = Vec::new();
    let mut out_lines: Vec<String> = Vec::new();
    let mut used_slugs: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    // Estado de bloque de código por vallas: guardamos el delimitador activo.
    let mut fence: Option<char> = None;

    for line in markdown.lines() {
        let trimmed = line.trim_start();

        // Detección de apertura/cierre de bloques de código con vallas.
        if let Some(f) = fence {
            if trimmed.starts_with(&f.to_string().repeat(3)) {
                fence = None;
            }
            out_lines.push(line.to_string());
            continue;
        } else if trimmed.starts_with("```") {
            fence = Some('`');
            out_lines.push(line.to_string());
            continue;
        } else if trimmed.starts_with("~~~") {
            fence = Some('~');
            out_lines.push(line.to_string());
            continue;
        }

        // ¿Es una línea de encabezado ATX (`#`..`######` seguido de espacio)?
        if let Some((level, rest)) = parse_atx_heading(trimmed) {
            // Texto del encabezado, quitando un posible `{#id}` ya existente.
            let (visible, existing_id) = split_existing_id(rest);
            let text = visible.trim().to_string();

            let slug = if let Some(id) = existing_id {
                id
            } else {
                let base = slugify(&text);
                let unique = make_unique(&base, &mut used_slugs);
                // Inyectamos el id al final de la línea de encabezado original,
                // preservando la indentación de la línea.
                let indent = &line[..line.len() - trimmed.len()];
                out_lines.push(format!(
                    "{indent}{} {} {{#{unique}}}",
                    "#".repeat(level),
                    text
                ));
                headings.push(Heading { level, text, slug: unique.clone() });
                continue;
            };

            headings.push(Heading { level, text, slug });
            out_lines.push(line.to_string());
            continue;
        }

        out_lines.push(line.to_string());
    }

    TocResult {
        rendered_markdown: out_lines.join("\n"),
        headings,
    }
}

/// Reconoce un encabezado ATX y devuelve (nivel, texto tras las almohadillas).
fn parse_atx_heading(trimmed: &str) -> Option<(usize, &str)> {
    let hashes = trimmed.chars().take_while(|&c| c == '#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = &trimmed[hashes..];
    // Debe haber al menos un espacio tras las almohadillas.
    if !rest.starts_with(' ') && !rest.is_empty() {
        return None;
    }
    Some((hashes, rest.trim_start()))
}

/// Separa un posible `{#id}` al final del texto del encabezado.
/// Devuelve (texto_visible, Some(id)) si existía, o (texto, None) si no.
fn split_existing_id(text: &str) -> (&str, Option<String>) {
    let t = text.trim_end();
    if t.ends_with('}') {
        if let Some(open) = t.rfind("{#") {
            let id = &t[open + 2..t.len() - 1];
            if !id.is_empty() {
                return (t[..open].trim_end(), Some(id.to_string()));
            }
        }
    }
    (text, None)
}

/// Convierte un texto de encabezado en un slug apto para ancla:
/// minúsculas, espacios a guiones, y solo caracteres alfanuméricos y guiones.
fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut prev_dash = false;
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            for lc in ch.to_lowercase() {
                slug.push(lc);
            }
            prev_dash = false;
        } else if ch.is_whitespace() || ch == '-' || ch == '_' {
            if !prev_dash && !slug.is_empty() {
                slug.push('-');
                prev_dash = true;
            }
        }
        // El resto de caracteres (puntuación, emojis, …) se descartan.
    }
    // Quita un guion final sobrante.
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        slug.push_str("seccion");
    }
    slug
}

/// Garantiza que el slug sea único añadiendo un sufijo -1, -2, … si se repite.
fn make_unique(base: &str, used: &mut std::collections::HashMap<String, usize>) -> String {
    match used.get_mut(base) {
        Some(count) => {
            *count += 1;
            format!("{base}-{count}")
        }
        None => {
            used.insert(base.to_string(), 0);
            base.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrae_encabezados_con_nivel_y_texto() {
        let md = "# Título\n\ntexto\n\n## Sub sección\n\n### Detalle";
        let r = process(md);
        assert_eq!(r.headings.len(), 3);
        assert_eq!(r.headings[0].level, 1);
        assert_eq!(r.headings[0].text, "Título");
        assert_eq!(r.headings[1].level, 2);
        assert_eq!(r.headings[1].text, "Sub sección");
        assert_eq!(r.headings[2].level, 3);
    }

    #[test]
    fn genera_slugs_correctos() {
        let md = "# Hola Mundo\n## Características (MVP)";
        let r = process(md);
        assert_eq!(r.headings[0].slug, "hola-mundo");
        assert_eq!(r.headings[1].slug, "características-mvp");
    }

    #[test]
    fn inyecta_id_en_la_linea_de_encabezado() {
        let md = "# Título";
        let r = process(md);
        assert!(r.rendered_markdown.contains("{#título}"));
    }

    #[test]
    fn respeta_id_existente() {
        let md = "# Título {#personalizado}";
        let r = process(md);
        assert_eq!(r.headings[0].slug, "personalizado");
        assert_eq!(r.headings[0].text, "Título");
    }

    #[test]
    fn ignora_almohadillas_en_bloques_de_codigo() {
        let md = "# Real\n\n```bash\n# esto es un comentario, no un encabezado\n```\n\n## Otra";
        let r = process(md);
        assert_eq!(r.headings.len(), 2);
        assert_eq!(r.headings[0].text, "Real");
        assert_eq!(r.headings[1].text, "Otra");
    }

    #[test]
    fn desambigua_slugs_duplicados() {
        let md = "# Uso\n## Uso\n### Uso";
        let r = process(md);
        assert_eq!(r.headings[0].slug, "uso");
        assert_eq!(r.headings[1].slug, "uso-1");
        assert_eq!(r.headings[2].slug, "uso-2");
    }
}

