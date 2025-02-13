use crate::debugger::Debugger;

const DEFAULT_ROWS: usize = 100;
const FONT_SIZE: f32 = 12.0;

#[derive(Default)]
pub struct Editor {
    pub code: String,
    pub selected_line: usize,

    prev_cursor_pos: usize,
}

impl Editor {
    fn name(&self) -> &'static str {
        "Editor"
    }
    pub fn show(&mut self, ui: &mut egui::Ui, _debugger: &mut Option<Debugger>) {
        // TODO: Draw an arrow for where the program counter is and breakpoint functionality

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_top(|h| {
                self.numbering(h, &self.code);

                let available_width = h.available_width();
                let output = egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(DEFAULT_ROWS)
                    .lock_focus(true)
                    // TODO: Subtract the available width by the difference taken by the line numbering.
                    .desired_width(available_width - (available_width / 23.0))
                    .show(h);

                // Keep track of the current line being selected, only update when changed.
                if let Some(cursor_range) = output.cursor_range {
                    let cursor_pos = cursor_range.primary.ccursor.index;

                    if cursor_pos != self.prev_cursor_pos {
                        self.selected_line = self.code[..cursor_pos].matches('\n').count();
                        self.prev_cursor_pos = cursor_pos;
                    }
                }
            });
        });
    }
}

impl Editor {
    fn numbering(&self, ui: &mut egui::Ui, code: &str) {
        let total = code.lines().count();
        let max_ident = total.to_string().len();
        let mut counter = (1..=total)
            .map(|i| {
                let label = i.to_string();
                format!(
                    "{}{label}",
                    " ".repeat(max_ident.saturating_sub(label.len()))
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let width = max_ident as f32 * FONT_SIZE * 0.5;

        let mut layouter = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
            let layout_job = egui::text::LayoutJob::single_section(
                string.to_string(),
                egui::TextFormat::simple(egui::FontId::monospace(FONT_SIZE), egui::Color32::GRAY),
            );
            ui.fonts(|f| f.layout_job(layout_job))
        };

        ui.add(
            egui::TextEdit::multiline(&mut counter)
                .id_source(format!("{}_ln_breakpoints", self.name()))
                .font(egui::TextStyle::Monospace)
                .interactive(false)
                .frame(false)
                .desired_rows(DEFAULT_ROWS)
                .desired_width(width)
                .layouter(&mut layouter),
        );
    }
}
