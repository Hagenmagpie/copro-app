// 启用所有的 Clippy lint 和 Rust 2018 特性的警告
#![warn(clippy::all, rust_2018_idioms)]
// 如果不是在 debug 断言模式下，设置 windows_subsystem 为 "windows"，在 Windows 的 release 模式下隐藏控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 当以本地方式编译时：
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // 初始化 env_logger，如果你以 `RUST_LOG=debug` 运行，将日志输出到 stderr
    env_logger::init();

    // 设置本地运行选项
    let native_options = eframe::NativeOptions {
        // 设置视口
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]) // 设置内部大小
            .with_min_inner_size([300.0, 220.0]) // 设置最小内部大小
            .with_icon( // 设置图标，这是可选的
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default() // 使用默认值填充其余选项
    };
    // 运行本地应用
    eframe::run_native(
        "copro", // 应用名称
        native_options, // 本地运行选项
        Box::new(|cc| Box::new(copro::CoproApp::new(cc))), // 创建应用实例
    )
}

// 当使用 trunk 编译到 web 时：
#[cfg(target_arch = "wasm32")]
fn main() {
    // 将 `log` 消息重定向到 `console.log` 和其他相关函数
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    // 设置 web 运行选项
    let web_options = eframe::WebOptions::default();

    // 创建并启动 web 运行器
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // 硬编码 canvas id
                web_options, // web 运行选项
                Box::new(|cc| Box::new(copro::CoproApp::new(cc))), // 创建应用实例
            )
            .await
            .expect("failed to start eframe"); // 如果启动失败，抛出错误
    });
}
