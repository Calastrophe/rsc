use egui::Ui;

use crate::{
    debugger::{Debugger, ExecutionState},
    emulator::Assembler,
};

const FONT_SIZE: f32 = 17.0;

#[derive(Default)]
pub struct Top {}

impl Top {
    fn name(&self) -> &'static str {
        "Top"
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        debugger: &mut Option<Debugger>,
        assembler: &mut Option<Assembler>,
        code: &str,
    ) {
        ui.add_space(1.0);
        ui.horizontal(|ui| {
            if ui
                .button(egui::RichText::new("🔧").font(egui::FontId::monospace(FONT_SIZE)))
                .on_hover_text("Assemble")
                .clicked()
            {
                let new_assembler = Assembler::parse(code.to_string());
                // TODO: Spawn the debugger on another thread.
                if new_assembler.errors().is_empty() {
                    debugger.replace(Debugger::new(new_assembler.instructions()));
                }
                assembler.replace(new_assembler);
            };

            if let Some(debugger) = debugger {
                let (
                    pause_enabled,
                    run_enabled,
                    step_forward_enabled,
                    step_backward_enabled,
                    step_over_enabled,
                    restart_enabled,
                ) = match debugger.execution_state {
                    ExecutionState::Start => (false, true, true, false, false, false),
                    ExecutionState::Halted => (false, false, false, true, false, true),
                    ExecutionState::Paused | ExecutionState::Stepping => {
                        (false, true, true, true, false, true)
                    }
                    ExecutionState::Running => (true, false, false, false, false, false),
                    ExecutionState::BreakpointHit => (false, false, false, true, true, true),
                };

                ui.add_enabled_ui(pause_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("⏸").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Pause")
                        .on_disabled_hover_text("Pause")
                        .clicked()
                    {};
                });

                ui.add_enabled_ui(run_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("▶").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Run")
                        .on_disabled_hover_text("Run")
                        .clicked()
                    {};
                });

                ui.add_enabled_ui(step_backward_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("⬅").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Step Backward")
                        .on_disabled_hover_text("Step Backward")
                        .clicked()
                    {};
                });

                ui.add_enabled_ui(step_forward_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("➡").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Step Forward")
                        .on_disabled_hover_text("Step Forward")
                        .clicked()
                    {};
                });

                ui.add_enabled_ui(step_over_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("⤵").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Step Over")
                        .on_disabled_hover_text("Step Over")
                        .clicked()
                    {};
                });

                ui.add_enabled_ui(restart_enabled, |ui| {
                    if ui
                        .button(
                            egui::RichText::new("🔄").font(egui::FontId::proportional(FONT_SIZE)),
                        )
                        .on_hover_text("Restart")
                        .on_disabled_hover_text("Restart")
                        .clicked()
                    {};
                });

                ui.add(
                    egui::Slider::new::<u32>(&mut debugger.instructions_per_second, 1..=20)
                        .text("IPS"),
                );
            }
        });
        ui.add_space(1.0);
    }
}
