use eframe::egui::{self, Color32, CursorIcon, Pos2, Rounding, Stroke, Vec2};

pub fn ios_switch(ui: &mut egui::Ui, on: &mut bool) -> bool {
    let desired_size = Vec2::new(36.0, 20.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.hovered() {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }

    let mut changed = false;
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
        changed = true;
    }

    if ui.is_rect_visible(rect) {
        let how_on = if *on { 1.0 } else { 0.0 };

        let bg = if *on {
            Color32::from_rgb(52, 199, 89)
        } else {
            Color32::from_rgba_premultiplied(75, 78, 88, 180)
        };

        // Pure clean pill: ZERO white stroke outline
        ui.painter().rect_filled(rect, Rounding::same(10.0), bg);

        let circle_x = egui::lerp((rect.left() + 10.0)..=(rect.right() - 10.0), how_on);
        let center = Pos2::new(circle_x, rect.center().y);

        // Clean white thumb with subtle soft shadow
        ui.painter().circle_filled(
            Pos2::new(center.x, center.y + 0.5),
            7.5,
            Color32::from_rgba_premultiplied(0, 0, 0, 35),
        );
        ui.painter().circle_filled(center, 7.5, Color32::WHITE);
    }

    changed
}

pub fn service_row(
    ui: &mut egui::Ui,
    title: &str,
    subtitle: &str,
    on: &mut bool,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(13.5)
                    .strong()
                    .color(Color32::from_rgb(250, 250, 255)),
            );
            ui.label(
                egui::RichText::new(subtitle)
                    .size(10.5)
                    .color(Color32::from_rgb(175, 180, 190)),
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ios_switch(ui, on) {
                changed = true;
            }
        });
    });

    changed
}

pub fn draw_divider(ui: &mut egui::Ui) {
    ui.add_space(6.0);
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        Stroke::new(0.5_f32, Color32::from_rgba_premultiplied(255, 255, 255, 14)),
    );
    ui.add_space(6.0);
}

pub fn draw_traffic_lights(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let (close_rect, close_resp) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::click());
        let center = close_rect.center();
        let hovered = close_resp.hovered();

        if hovered {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        let close_col = if hovered {
            Color32::from_rgb(255, 75, 65)
        } else {
            Color32::from_rgb(255, 95, 86)
        };
        ui.painter().circle_filled(center, 6.0, close_col);

        let cross_col = if hovered {
            Color32::from_rgb(85, 0, 0)
        } else {
            Color32::from_rgba_premultiplied(85, 0, 0, 120)
        };
        let d = 2.8_f32;
        ui.painter().line_segment(
            [Pos2::new(center.x - d, center.y - d), Pos2::new(center.x + d, center.y + d)],
            Stroke::new(1.1_f32, cross_col),
        );
        ui.painter().line_segment(
            [Pos2::new(center.x + d, center.y - d), Pos2::new(center.x - d, center.y + d)],
            Stroke::new(1.1_f32, cross_col),
        );

        if close_resp.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }

        ui.add_space(4.0);

        let (min_rect, min_resp) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), egui::Sense::click());
        let min_center = min_rect.center();
        let min_hovered = min_resp.hovered();

        if min_hovered {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        let min_col = if min_hovered {
            Color32::from_rgb(255, 205, 45)
        } else {
            Color32::from_rgb(255, 189, 46)
        };
        ui.painter().circle_filled(min_center, 6.0, min_col);

        let minus_col = if min_hovered {
            Color32::from_rgb(155, 90, 0)
        } else {
            Color32::from_rgba_premultiplied(155, 90, 0, 120)
        };
        ui.painter().line_segment(
            [Pos2::new(min_center.x - 3.2_f32, min_center.y), Pos2::new(min_center.x + 3.2_f32, min_center.y)],
            Stroke::new(1.2_f32, minus_col),
        );

        if min_resp.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
    });
}
