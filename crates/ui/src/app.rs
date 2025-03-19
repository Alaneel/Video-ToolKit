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
    batch_tab::BatchTab,
    profiles_tab::ProfilesTab,
    plugins_tab::PluginsTab,
};

#[derive(PartialEq)]
pub enum Tab {
    Clipper,
    GifConverter,
    GifTransparency,
    Splitter,
    Merger,
    Batch,      // New tab
    Profiles,   // New tab
    Plugins,    // New tab
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
    batch_tab: BatchTab,           // New tab
    profiles_tab: ProfilesTab,     // New tab
    plugins_tab: PluginsTab,       // New tab
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
            batch_tab: BatchTab::new(Arc::clone(&status), Arc::clone(&processing)),
            profiles_tab: ProfilesTab::new(Arc::clone(&status), Arc::clone(&processing)),
            plugins_tab: PluginsTab::new(Arc::clone(&status), Arc::clone(&processing)),
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
                // Main operations tabs
                ui.selectable_value(&mut self.active_tab, Tab::Clipper, "Clip Video");
                ui.selectable_value(&mut self.active_tab, Tab::GifConverter, "Convert to GIF");
                ui.selectable_value(&mut self.active_tab, Tab::GifTransparency, "GIF Transparency");
                ui.selectable_value(&mut self.active_tab, Tab::Splitter, "Split Video");
                ui.selectable_value(&mut self.active_tab, Tab::Merger, "Merge Audio/Video");

                // Separator
                ui.separator();

                // Advanced features tabs
                ui.selectable_value(&mut self.active_tab, Tab::Batch, "Batch Processing");
                ui.selectable_value(&mut self.active_tab, Tab::Profiles, "Profiles");
                ui.selectable_value(&mut self.active_tab, Tab::Plugins, "Plugins");
            });

            ui.separator();

            // Tab content
            match self.active_tab {
                Tab::Clipper => self.clipper_tab.ui(ui),
                Tab::GifConverter => self.gif_converter_tab.ui(ui),
                Tab::GifTransparency => self.gif_transparency_tab.ui(ui),
                Tab::Splitter => self.splitter_tab.ui(ui),
                Tab::Merger => self.merger_tab.ui(ui),
                Tab::Batch => self.batch_tab.ui(ui),
                Tab::Profiles => self.profiles_tab.ui(ui),
                Tab::Plugins => self.plugins_tab.ui(ui),
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