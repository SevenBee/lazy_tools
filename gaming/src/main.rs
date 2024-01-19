#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::default;

use eframe::egui;

/// 一个ui界面
/// 干员列表界面
/// 背包列表界面
/// 
/// 功能:
/// 1. 导出，导入 干员列表json
/// 
/// 
/// 连接手机后，让手机亮度调低
/// 启动游戏
/// 登录帐号
/// 更新信息状态
/// 界面设计可以参考  https://excalidraw.com/#room=200addd6deb5083cf62d,0NLOAaC86GvcJo_IwPC-xA

fn main() -> Result<(), eframe::Error>{
    env_logger::init();
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Gmaing", 
        options,
        Box::new(|cc|{
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<ArknightsApp>::default()
        }),
        
    )
}

struct  ArknightsApp{
    name: String,
    age: u32,
}

impl Default for ArknightsApp {
    fn default() -> Self {
        Self { name: "doctor".to_owned(), age: 42, }
    }
}

impl eframe::App for ArknightsApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading(String::from("Arknights 种草机"));
            ui.horizontal(|ui|{
                let name_label = ui.label("you_name: ");
                let my_name = ui.label("seven");
                    // ui.text_edit_singleline(&mut self.name).labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("click each year").clicked(){
                self.age += 1;
            };
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}