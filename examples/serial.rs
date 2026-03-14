use heapless::Vec;
use alloc::{format, string::String};

pub fn run(log_list: &Vec<String, 12>) -> isize {
    let mut prev = eadkp::input::KeyboardState::scan();
    let line_height = eadkp::SMALL_FONT.height as usize + 2;
    let max_chars = (eadkp::SCREEN_RECT.width as usize / eadkp::SMALL_FONT.width as usize).max(4);
    let max_lines = ((eadkp::SCREEN_RECT.height as usize).saturating_sub(10) / line_height).max(1);
    let mut selected_index: Option<usize> = None;
    let mut scroll_start: usize = 0;
    let mut show_caret = false;
    let mut dirty = true;
    let mut h_offset: usize = 0;
    let mut h_offsets: Vec<usize, 12> = Vec::new();
    let mut last_status = String::new();

    while h_offsets.len() < log_list.len() {
        let _ = h_offsets.push(0);
    }

    loop {
        let now = eadkp::input::KeyboardState::scan();
        let just = now.get_just_pressed(prev);
        if just.key_down(eadkp::input::Key::Home) {
            break 0;
        };
        let up = just.key_down(eadkp::input::Key::Up);
        let down = just.key_down(eadkp::input::Key::Down);
        let left = just.key_down(eadkp::input::Key::Left);
        let right = just.key_down(eadkp::input::Key::Right);

        let old_selected = selected_index;
        let old_scroll_start = scroll_start;
        let old_show_caret = show_caret;
        let old_h_offset = h_offset;

        let total_lines = log_list.len();
        let mut new_selected = selected_index;
        let mut new_scroll_start = scroll_start;
        let mut new_show_caret: bool;
        let mut available_lines = max_lines;

        while h_offsets.len() < total_lines {
            let _ = h_offsets.push(0);
        }

        if total_lines == 0 {
            new_selected = None;
            new_scroll_start = 0;
            h_offset = 0;
        }

        if up && total_lines > 0 {
            new_selected = match new_selected {
                None => Some(total_lines - 1),
                Some(0) => Some(0),
                Some(i) => Some(i - 1),
            };
        }

        if down {
            if let Some(i) = new_selected {
                if i + 1 < total_lines {
                    new_selected = Some(i + 1);
                }
            }
        }

        if let Some(_) = new_selected {
            if left {
                h_offset = h_offset.saturating_sub(1);
            }

            if right {
                h_offset = h_offset.saturating_add(1);
            }
        }

        if let Some(i) = new_selected {
            if i < new_scroll_start {
                new_scroll_start = i;
            } else if i >= new_scroll_start + available_lines {
                new_scroll_start = i + 1 - available_lines;
            }
        } else {
            new_scroll_start = total_lines.saturating_sub(available_lines);
        }

        new_show_caret = new_scroll_start > 0;
        if new_show_caret && max_lines > 1 {
            available_lines = max_lines - 1;
            if let Some(i) = new_selected {
                if i < new_scroll_start {
                    new_scroll_start = i;
                } else if i >= new_scroll_start + available_lines {
                    new_scroll_start = i + 1 - available_lines;
                }
            } else {
                new_scroll_start = total_lines.saturating_sub(available_lines);
            }
            new_show_caret = new_scroll_start > 0;
        }

        if new_selected != selected_index {
            if let Some(old) = selected_index {
                if old < h_offsets.len() {
                    h_offsets[old] = h_offset;
                }
            }
            if let Some(i) = new_selected {
                h_offset = if i < h_offsets.len() { h_offsets[i] } else { 0 };
            } else {
                h_offset = 0;
            }
        }

        if let Some(i) = new_selected {
            if let Some(msg) = log_list.get(i) {
                let line_len = msg.chars().count();
                let max_offset = line_len.saturating_sub(max_chars.saturating_sub(3));
                if line_len <= max_chars {
                    h_offset = 0;
                } else if h_offset > max_offset {
                    h_offset = max_offset;
                }
            }
            if i < h_offsets.len() {
                h_offsets[i] = h_offset;
            }
        }

        let selection_bg = eadkp::Color::from_888(230, 230, 230);
        let y_base = if new_show_caret { 10 + line_height } else { 10 };
        let end = (new_scroll_start + available_lines).min(total_lines);
        let full_redraw = new_scroll_start != old_scroll_start || new_show_caret != old_show_caret || dirty;
        let hscroll_changed = h_offset != old_h_offset;
        let selection_changed = new_selected != old_selected;
        let mut status = String::new();
        if let Some(i) = new_selected {
            status = format!("{}/{}", i + 1, total_lines);
        }
        let status_changed = status != last_status;

        selected_index = new_selected;
        scroll_start = new_scroll_start;
        show_caret = new_show_caret;
        dirty = false;

        let render_line = |index: usize| {
            if index < scroll_start || index >= end {
                return;
            }
            if let Some(msg) = log_list.get(index) {
                let is_selected = selected_index == Some(index);
                let bg = if is_selected { selection_bg } else { eadkp::COLOR_WHITE };
                let mut line = String::new();
                let mut show_left = false;
                let mut show_right = false;

                let line_len = msg.chars().count();
                if line_len <= max_chars {
                    line.push_str(msg.as_str());
                } else {
                    let use_offset = if index < h_offsets.len() { h_offsets[index] } else { 0 };
                    let left_ellipsis = use_offset > 0;
                    let max_offset = line_len.saturating_sub(max_chars.saturating_sub(3));
                    let use_offset = use_offset.min(max_offset);
                    let right_ellipsis = use_offset + max_chars < line_len;
                    let mut content_width = max_chars;
                    if left_ellipsis {
                        content_width = content_width.saturating_sub(1);
                        line.push(' ');
                    }
                    if right_ellipsis {
                        content_width = content_width.saturating_sub(3);
                    }

                    let mut count = 0usize;
                    for (i, ch) in msg.chars().enumerate() {
                        if i < use_offset {
                            continue;
                        }
                        if count >= content_width {
                            break;
                        }
                        line.push(ch);
                        count += 1;
                    }

                    if right_ellipsis {
                        line.push_str("...");
                    }

                    if is_selected {
                        show_left = left_ellipsis;
                        show_right = right_ellipsis;
                    }
                }

                let row = index - scroll_start;
                let y = y_base + row * line_height;
                eadkp::display::push_rect_uniform(
                    eadkp::Rect {
                        x: 0,
                        y: y as u16,
                        width: eadkp::SCREEN_RECT.width,
                        height: line_height as u16,
                    },
                    bg,
                );
                eadkp::display::draw_string(
                    line.as_str(),
                    eadkp::Point { y: y as u16, x: 5 },
                    false,
                    eadkp::COLOR_BLACK,
                    bg,
                );

                if is_selected && show_left {
                    eadkp::display::draw_string(
                        "<",
                        eadkp::Point { y: y as u16, x: 0 },
                        false,
                        eadkp::COLOR_BLACK,
                        bg,
                    );
                }
                if is_selected && show_right {
                    let x = eadkp::SCREEN_RECT.width.saturating_sub(eadkp::SMALL_FONT.width) as u16;
                    eadkp::display::draw_string(
                        ">",
                        eadkp::Point { y: y as u16, x },
                        false,
                        eadkp::COLOR_BLACK,
                        bg,
                    );
                }
            }
        };

        if full_redraw {
            if show_caret {
                eadkp::display::draw_string(
                    "^",
                    eadkp::Point { y: 2, x: 5 },
                    false,
                    eadkp::COLOR_BLACK,
                    eadkp::COLOR_WHITE,
                );
            } else {
                eadkp::display::push_rect_uniform(
                    eadkp::Rect {
                        x: 0,
                        y: 0,
                        width: eadkp::SCREEN_RECT.width,
                        height: line_height as u16,
                    },
                    eadkp::COLOR_WHITE,
                );
            }

            if !status.is_empty() {
                let status_x = eadkp::SCREEN_RECT.width as usize
                    - (status.chars().count() * eadkp::SMALL_FONT.width as usize)
                    - 2;
                eadkp::display::draw_string(
                    status.as_str(),
                    eadkp::Point { y: 2, x: status_x as u16 },
                    false,
                    eadkp::COLOR_BLACK,
                    eadkp::COLOR_WHITE,
                );
            }

            let mut y = y_base;
            for index in scroll_start..end {
                render_line(index);
                y += line_height;
            }

            let mut remaining = available_lines.saturating_sub(end.saturating_sub(scroll_start));
            while remaining > 0 {
                eadkp::display::push_rect_uniform(
                    eadkp::Rect {
                        x: 0,
                        y: y as u16,
                        width: eadkp::SCREEN_RECT.width,
                        height: line_height as u16,
                    },
                    eadkp::COLOR_WHITE,
                );
                y += line_height;
                remaining -= 1;
            }
        } else {
            if selection_changed {
                if let Some(old) = old_selected {
                    render_line(old);
                }
                if let Some(new_sel) = selected_index {
                    render_line(new_sel);
                }
            } else if hscroll_changed {
                if let Some(sel) = selected_index {
                    render_line(sel);
                }
            }

            if status_changed || show_caret != old_show_caret {
                let status_width = (last_status.chars().count() * eadkp::SMALL_FONT.width as usize) as u16;
                let status_x = eadkp::SCREEN_RECT.width.saturating_sub(status_width) as u16;
                eadkp::display::push_rect_uniform(
                    eadkp::Rect {
                        x: status_x,
                        y: 0,
                        width: (status_width + 4).min(eadkp::SCREEN_RECT.width),
                        height: line_height as u16,
                    },
                    eadkp::COLOR_WHITE,
                );
                if !status.is_empty() {
                    let x = eadkp::SCREEN_RECT.width as usize
                        - (status.chars().count() * eadkp::SMALL_FONT.width as usize)
                        - 2;
                    eadkp::display::draw_string(
                        status.as_str(),
                        eadkp::Point { y: 2, x: x as u16 },
                        false,
                        eadkp::COLOR_BLACK,
                        eadkp::COLOR_WHITE,
                    );
                }
            }
        }

        last_status = status;

        prev = now;
    }
}
