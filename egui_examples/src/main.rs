use eframe::{egui::{self, TextStyle, RichText, FontFamily}, epaint::FontId};

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";


fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions::default();
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
    //     ..Default::default()
    // };
    eframe::run_native(
        "egui_examples",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allow_to_cloes: bool,
    text: String,
}

fn heading2() -> TextStyle{
    TextStyle::Name("Heading2".into())
}
fn heading3() -> TextStyle{
    TextStyle::Name("ContextHeading".into())
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // TODO: 到时候设置中文可能会用到这里
    let mut fonts = egui::FontDefinitions::default();

    // fonts.font_data.insert(
    //     "my_font".to_owned(),
    //     egui::FontData::from_static(include_bytes!(
    //         "../../fonts/Sontti.tty"
    //     )),
    // );

    // // Put my font first (highest priority) for proportional text:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Proportional)
    //     .or_default()
    //     .insert(0, "my_font".to_owned());

    // // Put my font as last fallback for monospace:
    // fonts
    //     .families
    //     .entry(egui::FontFamily::Monospace)
    //     .or_default()
    //     .push("my_font".to_owned());

    // // Tell egui to use these fonts:
    // ctx.set_fonts(fonts);
}

fn configure_text_styles(ctx: &egui::Context){
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ].into();
    ctx.set_style(style);
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        Self {
            show_confirmation_dialog: false,
            allow_to_cloes: false,
            text: "Edit this text field if you want".to_owned(),
        }
    }
    fn check_close(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.viewport().close_requested()) {
            println!("{}", self.allow_to_cloes);
            if self.allow_to_cloes {
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
    
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to Quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if ui.button("No").clicked() {
                        self.show_confirmation_dialog = false;
                        self.allow_to_cloes = false
                    };
                    if ui.button("Yes").clicked() {
                        self.show_confirmation_dialog = false;
                        self.allow_to_cloes = true;
    
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    };
                });
        }
    }

    fn font_style(ui: &mut egui::Ui){
        ui.heading("Top Heading");
        ui.add_space(5.);
        ui.label(LOREM_IPSUM);
        ui.add_space(30.);
        ui.label(RichText::new("Level 2").text_style(heading2()).strong());
        ui.add_space(5.);
        ui.label(LOREM_IPSUM);
        ui.add_space(60.);
        ui.label(RichText::new("Level 3").text_style(heading3()).strong());
        ui.add_space(5.);
        ui.label(LOREM_IPSUM);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close window");
            ui.text_edit_multiline(&mut self.text);
            MyApp::font_style(ui);
        });
        // 判断是否需要关闭程序
        self.check_close(ctx);
    }
}

