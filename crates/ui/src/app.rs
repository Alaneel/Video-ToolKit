use eframe::{egui, epi};
use egui::{Color32, RichText, Ui};
use std::sync::{Arc, Mutex};

use common::check_ffmpeg;
use crate::tabs::{
    clipper_tab::ClipperTab,
    gif_converter_tab::GifConverterTab,
    splitter_tab::SplitterTab,
    merger_tab::MergerTab,
};

#[derive(PartialEq)]
pub enum Tab {
    Clipper,
    GifConverter,
    Splitter,
    Merger,
}

pub struct VideoToolkitApp {
    active_tab: Tab,
    status: Arc<Mutex<String>>,
    processing: Arc<Mutex<bool>>,

    clipper_tab: ClipperTab,
    gif_converter_tab: GifConverterTab,
    splitter_tab: SplitterTab,
    merger_tab: MergerTab,
}

impl Default for VideoToolkitApp {
    fn default() -> Self {
        let status = Arc::new(Mutex::new("Ready".to_string()));
        let processing = Arc::new(Mutex::new(false));

        Self {
            active_tab: Tab::Clipper,
            status: Arc::clone(&status),
            processing: Arc::clone(&processing),

            clipper_tab: ClipperTab::new(Arc::clone(&status), Arc::clone(&processing)),
            gif_converter_tab: GifConverterTab::new(Arc::clone(&status), Arc::clone(&processing)),
            splitter_tab: SplitterTab::new(Arc::clone(&status), Arc::clone(&processing)),
            merger_tab: MergerTab::new(Arc::clone(&status), Arc::clone(&processing)),
        }
    }
}

impl epi::App for VideoToolkitApp {
    fn name(&self) -> &str {
        "Video Toolkit"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Video Toolkit");

            // Check for FFmpeg
            if !check_ffmpeg() {
                ui.label(
                    RichText::new("Error: FFmpeg is not installed or not found in PATH. Please install FFmpeg.")
                        .color(Color32::RED)
                );
                return;
            }

            // Tab selector
            ui.horizontal(|ui| {
                if ui.selectable_label(self.active_tab == Tab::Clipper, "Clip Video").clicked() {
                    self.active_tab = Tab::Clipper;
                }
                if ui.selectable_label(self.active_tab == Tab::GifConverter, "Convert to GIF").clicked() {
                    self.active_tab = Tab::GifConverter;
                }
                if ui.selectable_label(self.active_tab == Tab::Splitter, "Split Video").clicked() {
                    self.active_tab = Tab::Splitter;
                }
                if ui.selectable_label(self.active_tab == Tab::Merger, "Merge Audio/Video").clicked() {
                    self.active_tab = Tab::Merger;
                }
            });

            ui.separator();

            // Tab content
            match self.active_tab {
                Tab::Clipper => self.clipper_tab.ui(ui),
                Tab::GifConverter => self.gif_converter_tab.ui(ui),
                Tab::Splitter => self.splitter_tab.ui(ui),
                Tab::Merger => self.merger_tab.ui(ui),
            }

            // Status bar
            ui.separator();
            ui.horizontal(|ui| {
                let status = self.status.lock().unwrap().clone();
                ui.label(&status);

                if *self.processing.lock().unwrap() {
                    ui.spinner();
                }
            });
        });
    }
}