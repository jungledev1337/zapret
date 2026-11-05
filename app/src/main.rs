#![windows_subsystem = "windows"]

use zapret_lib::config::AppConfig;
use zapret_lib::engine::EngineManager;
use zapret_lib::ui;
use eframe::egui::{self, Color32, CursorIcon, Rounding, Vec2};
use raw_window_handle::HasWindowHandle;
use std::path::PathBuf;

struct ZapretApp {
    config: AppConfig,
    config_path: PathBuf,
    engine: EngineManager,
    base_dir: PathBuf,
}

impl ZapretApp {
    fn new() -> Self {
        let base_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        let config_path = base_dir.join("settings.json");
        let config = AppConfig::load_from(&config_path);

        let mut app = Self {
            config,
            config_path,
            engine: EngineManager::default(),
            base_dir,
        };

        app.apply();
        app
    }

    fn apply(&mut self) {
        let _ = self.config.save_to(&self.config_path);
        let _ = self.engine.start(
            &self.base_dir,
            self.config.discord,
            self.config.telegram,
            self.config.youtube,
        );
    }
}

impl Drop for ZapretApp {
    fn drop(&mut self) {
        self.engine.stop();
    }
}

impl eframe::App for ZapretApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visual = egui::Visuals::dark();
        visual.panel_fill = Color32::TRANSPARENT;
        visual.window_stroke = egui::Stroke::NONE;
        visual.widgets.noninteractive.bg_stroke = egui::Stroke::NONE;
        visual.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        visual.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        visual.widgets.active.bg_stroke = egui::Stroke::NONE;
        ctx.set_visuals(visual);

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui_ctx| {
                let window_rect = ui_ctx.max_rect();

                // Soft background tint to strengthen the blur diffusion (Zero white stroke/corners)
                ui_ctx.painter().rect_filled(
                    window_rect,
                    Rounding::ZERO,
                    Color32::from_rgba_premultiplied(16, 18, 24, 60),
                );

                ui_ctx.allocate_new_ui(egui::UiBuilder::new().max_rect(window_rect.shrink(14.0)), |ui| {
                    // Header Bar
                    ui.horizontal(|ui| {
                        ui::draw_traffic_lights(ui);

                        ui.add_space(8.0);

                        // Draggable Area for Window Move
                        let drag_width = ui.available_width() - 50.0;
                        let (drag_rect, drag_resp) = ui.allocate_exact_size(
                            Vec2::new(drag_width.max(10.0), 22.0),
                            egui::Sense::click_and_drag(),
                        );

                        if drag_resp.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::Grab);
                        }
                        if drag_resp.drag_started() || drag_resp.dragged() {
                            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }

                        // Title inside drag area
                        ui.painter().text(
                            drag_rect.left_center(),
                            egui::Align2::LEFT_CENTER,
                            "zapret",
                            egui::FontId::proportional(14.0),
                            Color32::WHITE,
                        );

                        ui.painter().text(
                            drag_rect.left_center() + Vec2::new(46.0, 1.0),
                            egui::Align2::LEFT_CENTER,
                            "@jungle_dev",
                            egui::FontId::proportional(10.5),
                            Color32::from_rgb(180, 185, 200),
                        );

                        // Status Indicator
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let is_active = self.engine.is_running();
                            let (dot_color, label_text) = if is_active {
                                (Color32::from_rgb(52, 215, 88), "active")
                            } else {
                                (Color32::from_rgb(160, 165, 175), "off")
                            };

                            ui.label(
                                egui::RichText::new(label_text)
                                    .size(10.5)
                                    .strong()
                                    .color(dot_color),
                            );

                            let (dot_rect, _) = ui.allocate_exact_size(Vec2::new(6.0, 6.0), egui::Sense::hover());
                            ui.painter().circle_filled(dot_rect.center(), 2.5, dot_color);
                        });
                    });

                    ui.add_space(10.0);

                    // Liquid Glass Content
                    let mut changed = false;

                    if ui::service_row(ui, "Discord", "Voice RTC & Gateway", &mut self.config.discord) {
                        changed = true;
                    }

                    ui::draw_divider(ui);

                    if ui::service_row(ui, "Telegram", "Web & MTProto DC1-DC5", &mut self.config.telegram) {
                        changed = true;
                    }

                    ui::draw_divider(ui);

                    if ui::service_row(ui, "YouTube", "GoogleVideo Streams", &mut self.config.youtube) {
                        changed = true;
                    }

                    if changed {
                        self.apply();
                    }
                });
            });
    }
}

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([290.0, 205.0])
        .with_resizable(false)
        .with_maximize_button(false)
        .with_transparent(true)
        .with_decorations(false)
        .with_title("zapret | @jungle_dev");

    eframe::run_native(
        "zapret",
        options,
        Box::new(|cc| {
            #[cfg(target_os = "windows")]
            {
                if let Ok(handle) = cc.window_handle() {
                    if let raw_window_handle::RawWindowHandle::Win32(win32) = handle.as_raw() {
                        let hwnd = win32.hwnd.get() as windows_sys::Win32::Foundation::HWND;
                        unsafe {
                            // DWMWCP_ROUND (2)
                            let corner_pref: u32 = 2;
                            let _ = windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
                                hwnd,
                                33,
                                &corner_pref as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );

                            // DWMSBT_TRANSIENTWINDOW (3) = Full Acrylic Blur
                            let backdrop_pref: u32 = 3;
                            let _ = windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
                                hwnd,
                                38,
                                &backdrop_pref as *const _ as *const _,
                                std::mem::size_of::<u32>() as u32,
                            );
                        }
                    }

                    // Deep Frosted Glass: alpha 95 activates the deep diffusion kernel
                    let _ = window_vibrancy::apply_acrylic(&handle, Some((18, 20, 26, 95)));
                }
            }

            Ok(Box::new(ZapretApp::new()))
        }),
    )
}
