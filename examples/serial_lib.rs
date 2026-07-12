use heapless::Vec;
use alloc::string::{String, ToString};
use alloc::format;

const LINE_PADDING: usize = 2;

/// Compte le nombre de lignes (`\n`-séparées) dans un message.
fn count_rows(msg: &str) -> usize {
    msg.bytes().filter(|&b| b == b'\n').count() + 1
}

/// Retourne la longueur de la ligne la plus longue dans un message.
fn max_line_len(msg: &str) -> usize {
    msg.split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

/// Ajuste `scroll_start` pour que `selected` soit visible dans `available_rows` lignes.
fn adjust_scroll<const N: usize>(
    log_list: &Vec<String, N>,
    selected: Option<usize>,
    scroll_start: &mut usize,
    available_rows: usize,
) {
    let total = log_list.len();
    if total == 0 {
        *scroll_start = 0;
        return;
    }

    match selected {
        Some(sel) => {
            if sel < *scroll_start {
                *scroll_start = sel;
                return;
            }
            while *scroll_start < sel {
                let rows: usize = (*scroll_start..=sel)
                    .filter_map(|i| log_list.get(i))
                    .map(|msg| count_rows(msg.as_str()))
                    .sum();
                if rows <= available_rows { break; }
                *scroll_start += 1;
            }
        }
        None => {
            // Pas de sélection → scroll vers le bas
            let mut start = total;
            let mut rows = 0usize;
            while start > 0 {
                rows += log_list.get(start - 1).map_or(0, |m| count_rows(m.as_str()));
                if rows >= available_rows { break; }
                start -= 1;
            }
            *scroll_start = start;
        }
    }
}

/// Dessine un segment de texte à la position `y`,
/// avec scrolling horizontal et ellipsis (`<` / `...`) si nécessaire.
fn draw_segment(
    segment: &str,
    y: usize,
    is_selected: bool,
    h_offset: usize,
    max_chars: usize,
    line_height: usize,
    selection_bg: eadkp::Color,
) {
    let bg = if is_selected { selection_bg } else { eadkp::COLOR_WHITE };
    let line_len = segment.chars().count();

    let (line, show_left, show_right) = if line_len <= max_chars {
        (segment.to_string(), false, false)
    } else {
        let max_offset     = line_len.saturating_sub(max_chars.saturating_sub(3));
        let use_offset     = h_offset.min(max_offset);
        let left_ellipsis  = use_offset > 0;
        let right_ellipsis = use_offset + max_chars < line_len;

        // Largeur utile = max_chars - 1 (si gauche) - 3 (si droite)
        let content_width = max_chars
            .saturating_sub(left_ellipsis as usize)
            .saturating_sub(if right_ellipsis { 3 } else { 0 });

        let mut buf = String::new();
        if left_ellipsis { buf.push(' '); }

        let mut count = 0;
        for (i, ch) in segment.chars().enumerate() {
            if i < use_offset { continue; }
            if count >= content_width { break; }
            buf.push(ch);
            count += 1;
        }
        if right_ellipsis { buf.push_str("..."); }

        (buf, is_selected && left_ellipsis, is_selected && right_ellipsis)
    };

    eadkp::display::push_rect_uniform(
        eadkp::Rect { x: 0, y: y as u16, width: eadkp::SCREEN_RECT.width, height: line_height as u16 },
        bg,
    );
    eadkp::display::draw_string(
        &line,
        eadkp::Point { x: 5, y: y as u16 },
        false,
        eadkp::COLOR_BLACK,
        bg,
    );
    if show_left {
        eadkp::display::draw_string("<", eadkp::Point { x: 0, y: y as u16 }, false, eadkp::COLOR_BLACK, bg);
    }
    if show_right {
        let x = eadkp::SCREEN_RECT.width.saturating_sub(eadkp::SMALL_FONT.width);
        eadkp::display::draw_string(">", eadkp::Point { x: x as u16, y: y as u16 }, false, eadkp::COLOR_BLACK, bg);
    }
}

/// Affiche une liste de logs avec navigation clavier.
/// Générique sur la capacité du Vec — compatible avec toute taille `N`.
pub fn run<const N: usize>(log_list: &Vec<String, N>) -> isize {
    let line_height  = eadkp::SMALL_FONT.height as usize + LINE_PADDING;
    let max_chars    = (eadkp::SCREEN_RECT.width as usize / eadkp::SMALL_FONT.width as usize).max(4);
    let max_lines    = ((eadkp::SCREEN_RECT.height as usize).saturating_sub(10) / line_height).max(1);
    let selection_bg = eadkp::Color::from_888(230, 230, 230);

    let mut prev           = eadkp::input::KeyboardState::scan();
    let mut selected_index: Option<usize> = None;
    let mut scroll_start   = 0usize;
    let mut show_caret     = false;
    let mut dirty          = true; // force un rendu complet sur le premier frame
    let mut h_offset       = 0usize;
    let mut h_offsets: Vec<usize, N> = Vec::new();
    let mut last_status    = String::new();

    loop {
        let now  = eadkp::input::KeyboardState::scan();
        let just = now.get_just_pressed(prev);

        if just.key_down(eadkp::input::Key::Home) {
            return 0;
        }

        let total_lines = log_list.len();
        while h_offsets.len() < total_lines {
            let _ = h_offsets.push(0);
        }

        // ── Snapshot avant modifications (pour détection de changements) ──
        let old_selected = selected_index;
        let old_h_offset = h_offset;
        let old_scroll   = scroll_start;
        let old_caret    = show_caret;

        // ── Navigation verticale ──────────────────────────────────────────
        let mut new_selected = selected_index;

        if just.key_down(eadkp::input::Key::Up) && total_lines > 0 {
            new_selected = Some(match new_selected {
                None | Some(0) => 0,
                Some(i) => i - 1,
            });
        }
        if just.key_down(eadkp::input::Key::Down) {
            if let Some(i) = new_selected {
                if i + 1 < total_lines {
                    new_selected = Some(i + 1);
                }
            }
        }

        // ── Navigation horizontale ────────────────────────────────────────
        if new_selected.is_some() {
            if just.key_down(eadkp::input::Key::Left)  { h_offset = h_offset.saturating_sub(1); }
            if just.key_down(eadkp::input::Key::Right) { h_offset += 1; }
        }

        // ── Changement de sélection → sauvegarde / restauration h_offset ─
        if new_selected != old_selected {
            if let Some(old) = old_selected {
                if old < h_offsets.len() {
                    h_offsets[old] = h_offset;
                }
            }
            h_offset = new_selected
                .and_then(|i| h_offsets.get(i).copied())
                .unwrap_or(0);
        }

        // ── Clamping du h_offset sur la ligne sélectionnée ───────────────
        if let Some(i) = new_selected {
            if let Some(msg) = log_list.get(i) {
                let max_len = max_line_len(msg);
                h_offset = if max_len <= max_chars {
                    0
                } else {
                    h_offset.min(max_len.saturating_sub(max_chars.saturating_sub(3)))
                };
            }
            if i < h_offsets.len() {
                h_offsets[i] = h_offset;
            }
        }

        // ── Calcul du scroll et du caret ──────────────────────────────────
        let mut available_lines = max_lines;
        let mut new_scroll = scroll_start;

        adjust_scroll(log_list, new_selected, &mut new_scroll, available_lines);
        let mut new_caret = new_scroll > 0;
        if new_caret && max_lines > 1 {
            available_lines = max_lines - 1;
            adjust_scroll(log_list, new_selected, &mut new_scroll, available_lines);
            new_caret = new_scroll > 0;
        }

        // ── Statut ("3/12") ───────────────────────────────────────────────
        let status: String = new_selected
            .map(|i| format!("{}/{}", i + 1, total_lines))
            .unwrap_or_default();
        let status_changed = status != last_status;

        // ── Détection full redraw (avant commit pour capturer dirty) ──────
        let full_redraw = new_scroll != old_scroll
            || new_caret != old_caret
            || dirty
            || new_selected != old_selected
            || h_offset != old_h_offset;

        // ── Commit de l'état ──────────────────────────────────────────────
        selected_index = new_selected;
        scroll_start   = new_scroll;
        show_caret     = new_caret;
        dirty          = false;

        let y_base = if show_caret { 10 + line_height } else { 10 };

        // ── Rendu complet ─────────────────────────────────────────────────
        if full_redraw {
            // En-tête : caret ↑ ou espace blanc
            if show_caret {
                eadkp::display::draw_string(
                    "^",
                    eadkp::Point { x: 5, y: 2 },
                    false,
                    eadkp::COLOR_BLACK,
                    eadkp::COLOR_WHITE,
                );
            } else {
                eadkp::display::push_rect_uniform(
                    eadkp::Rect { x: 0, y: 0, width: eadkp::SCREEN_RECT.width, height: line_height as u16 },
                    eadkp::COLOR_WHITE,
                );
            }

            // Statut
            if !status.is_empty() {
                let sx = eadkp::SCREEN_RECT.width as usize
                    - status.chars().count() * eadkp::SMALL_FONT.width as usize - 2;
                eadkp::display::draw_string(
                    &status,
                    eadkp::Point { x: sx as u16, y: 2 },
                    false,
                    eadkp::COLOR_BLACK,
                    eadkp::COLOR_WHITE,
                );
            }

            // Lignes visibles
            let mut y         = y_base;
            let mut rows_left = available_lines;
            let mut index     = scroll_start;

            while index < total_lines && rows_left > 0 {
                if let Some(msg) = log_list.get(index) {
                    let is_selected = selected_index == Some(index);
                    let use_offset  = h_offsets.get(index).copied().unwrap_or(0);

                    for segment in msg.split('\n') {
                        if rows_left == 0 { break; }
                        draw_segment(segment, y, is_selected, use_offset, max_chars, line_height, selection_bg);
                        y += line_height;
                        rows_left -= 1;
                    }
                }
                index += 1;
            }

            // Remplissage blanc en bas
            while rows_left > 0 {
                eadkp::display::push_rect_uniform(
                    eadkp::Rect { x: 0, y: y as u16, width: eadkp::SCREEN_RECT.width, height: line_height as u16 },
                    eadkp::COLOR_WHITE,
                );
                y += line_height;
                rows_left -= 1;
            }

        // ── Mise à jour partielle du statut uniquement ────────────────────
        } else if status_changed {
            let old_w = (last_status.chars().count() * eadkp::SMALL_FONT.width as usize) as u16;
            let sx    = eadkp::SCREEN_RECT.width.saturating_sub(old_w);
            eadkp::display::push_rect_uniform(
                eadkp::Rect {
                    x: sx,
                    y: 0,
                    width: (old_w + 4).min(eadkp::SCREEN_RECT.width.saturating_sub(sx)),
                    height: line_height as u16,
                },
                eadkp::COLOR_WHITE,
            );
            if !status.is_empty() {
                let sx = eadkp::SCREEN_RECT.width as usize
                    - status.chars().count() * eadkp::SMALL_FONT.width as usize - 2;
                eadkp::display::draw_string(
                    &status,
                    eadkp::Point { x: sx as u16, y: 2 },
                    false,
                    eadkp::COLOR_BLACK,
                    eadkp::COLOR_WHITE,
                );
            }
        }

        last_status = status;
        prev = now;
    }
}