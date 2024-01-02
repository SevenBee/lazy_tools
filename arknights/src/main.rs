mod operator;
mod user_info;

use adb_helper::ADB;

static GAME_PACKAGE_NAME: &'static str = "com.hypergryph.arknights";
static GAME_LAUNCHABLE_ACTIVITY: &'static str = "com.u8.sdk.U8UnityContext";

#[tokio::main]
async fn main() {
    let adb = ADB::new().init();
    let device = &adb.devices[0];
    adb.start_app(device, GAME_PACKAGE_NAME, GAME_LAUNCHABLE_ACTIVITY)

    
}


fn login(){
    // 进入首页
    todo!()
}
