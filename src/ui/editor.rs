const DEFAULT_ROWS: usize = 100;
const FONT_SIZE: f32 = 12.0;

use super::Component;

#[derive(Default)]
pub struct Editor {
    code: String,
}

impl Component for Editor {
    fn name(&self) -> &'static str {
        "Editor"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_top(|h| {
                self.numbering(h, &self.code);

                let available_width = h.available_width();
                h.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(DEFAULT_ROWS)
                        .lock_focus(true)
                        .desired_width(available_width - (available_width / 4.0)),
                );
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
