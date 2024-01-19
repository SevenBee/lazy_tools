mod operator;    // 干员
mod user_info;  // 帐号信息
mod store_house; // 仓库
mod level; // 关卡
mod infrastructure; // 基建
mod ui; // GUI界面
mod database;
use adb_helper::ADB;
use rusqlite::Connection;

static GAME_PACKAGE_NAME: &'static str = "com.hypergryph.arknights";
static GAME_LAUNCHABLE_ACTIVITY: &'static str = "com.u8.sdk.U8UnityContext";

#[tokio::main]
async fn main() {
    // let adb = ADB::new().init();
    // let device = &adb.devices[0];
    // adb.get_current_activity(device);
    // adb.start_app(device, GAME_PACKAGE_NAME, GAME_LAUNCHABLE_ACTIVITY);



    
}

/// 进入首页
fn login(){
    todo!()
}

