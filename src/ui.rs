mod bytecode_viewer;
mod cpu;
mod editor;
mod top;
mod variable_viewer;

use crate::{debugger::Debugger, emulator::Assembler};
use egui::Ui;

use bytecode_viewer::BytecodeViewer;
use cpu::CpuState;
use editor::Editor;
use top::Top;
use variable_viewer::VariableViewer;

#[derive(Default)]
pub struct Interface {
    pub debugger: Option<Debugger>,
    pub assembler: Option<Assembler>,

    pub bytecode_viewer: BytecodeViewer,
    pub cpu_state: CpuState,
    pub editor: Editor,
    pub top: Top,
    pub variable_viewer: VariableViewer,
}

impl Interface {
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        Interface {
            ..Default::default()
        }
    }
}

impl eframe::App for Interface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.top.show(
                ui,
                &mut self.debugger,
                &mut self.assembler,
                &self.editor.code,
            );
        });

        let available_width = ctx.available_rect().width();
        let min_width = available_width / 6.0;

        egui::SidePanel::left("left_panel")
            .resizable(false)
            .min_width(min_width)
            .show(ctx, |ui| {
                self.bytecode_viewer
                    .show(ui, &self.debugger, &self.assembler);
            });

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(min_width)
            .show(ctx, |ui| {
                let (top, bottom) = ui
                    .available_rect_before_wrap()
                    .split_top_bottom_at_fraction(0.5);

                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(top), |ui| {
                    self.cpu_state.show(ui, &self.debugger);
                });

                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(bottom), |ui| {
                    self.variable_viewer
                        .show(ui, &self.debugger, &self.assembler);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.editor.show(ui, &mut self.debugger);
        });
    }
}
