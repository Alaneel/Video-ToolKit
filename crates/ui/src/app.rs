use eframe::egui;
use egui::{Color32, RichText};
use std::sync::{Arc, Mutex};

use common::check_ffmpeg;
use crate::tabs::{
    clipper_tab::ClipperTab,
    gif_converter_tab::GifConverterTab,
    gif_transparency_tab::GifTransparencyTab,
    splitter_tab::SplitterTab,
    merger_tab::MergerTab,
};

#[derive(PartialEq)]
pub enum Tab {
    Clipper,
    GifConverter,
    GifTransparency,
    Splitter,
    Merger,
}

pub struct VideoToolKitApp {
    active_tab: Tab,
    status: Arc<Mutex<String>>,
    processing: Arc<Mutex<bool>>,

    clipper_tab: ClipperTab,
    gif_converter_tab: GifConverterTab,
    gif_transparency_tab: GifTransparencyTab,
    splitter_tab: SplitterTab,
    merger_tab: MergerTab,
}

impl Default for VideoToolKitApp {
    fn default() -> Self {
        let status = Arc::new(Mutex::new("Ready".to_string()));
        let processing = Arc::new(Mutex::new(false));

        Self {
            active_tab: Tab::Clipper,
            status: Arc::clone(&status),
            processing: Arc::clone(&processing),

            clipper_tab: ClipperTab::new(Arc::clone(&status), Arc::clone(&processing)),
            gif_converter_tab: GifConverterTab::new(Arc::clone(&status), Arc::clone(&processing)),
            gif_transparency_tab: GifTransparencyTab::new(Arc::clone(&status), Arc::clone(&processing)),
            splitter_tab: SplitterTab::new(Arc::clone(&status), Arc::clone(&processing)),
            merger_tab: MergerTab::new(Arc::clone(&status), Arc::clone(&processing)),
        }
    }
}

impl eframe::App for VideoToolKitApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Video-ToolKit");

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
                if ui.selectable_label(self.active_tab == Tab::GifTransparency, "GIF Transparency").clicked() {
                    self.active_tab = Tab::GifTransparency;
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
                Tab::GifTransparency => self.gif_transparency_tab.ui(ui),
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