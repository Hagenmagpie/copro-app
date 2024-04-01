/// 我们派生Deserialize/Serialize以便在关闭时持久化应用状态。
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // 如果我们添加新的字段，当反序列化旧状态时给它们默认值
pub struct CoproApp {
    // 示例内容:
    label: String,

    #[serde(skip)] // 这是如何选择不序列化一个字段
    value: f32,
}

impl Default for CoproApp {
    fn default() -> Self {
        Self {
            // 示例内容:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl CoproApp {
    /// 在第一帧之前调用一次。
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 这也是你可以使用`cc.egui_ctx.set_visuals` 和 `cc.egui_ctx.set_fonts`自定义egui外观和感觉的地方。

        // 加载之前的应用状态（如果有）。
        // 注意，你必须启用`persistence`特性才能使这个工作。
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for CoproApp {
    /// 在关闭前由框架调用以保存状态。
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// 每次UI需要重绘时调用，这可能是每秒多次。
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 将你的小部件放入`SidePanel`，`TopBottomPanel`，`CentralPanel`，`Window`或`Area`。
        // 想要灵感和更多示例，去 https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // 顶部面板通常是放置菜单栏的好地方：

            egui::menu::bar(ui, |ui| {
                // 注意：网页上没有File->Quit！
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // 中央面板是添加了TopPanel's和SidePanel's后剩下的区域
            ui.heading("welcome to copro");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_copro/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
