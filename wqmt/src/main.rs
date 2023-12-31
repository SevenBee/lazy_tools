// mod user_info;
// mod friend;
// mod hero;

use adb_helper::{ADB, Device, Point, search_port};
// use friend::auto_friendship_points;

use std::{process::Command, thread::sleep, time::Duration};

fn main() {
    // let package = "";
    let adb = ADB::new().init();
    println!("设备:{:?}",adb.devices)
    // let device = &adb.devices[0];

    // // 登录并跳过广告
    // // adb.start_app(device, package)?;
    // if !check_login_page(){
    //     skip_bulletin_board(&adb, device)
    // }

    // // 一键领取友情点数
    // auto_friendship_points()
    // let process_list =  search_port(5037);
    // println!("{:?}",process_list);
    // if lines.is_empty() {
    //     println!("启动adb中...");
    //     Command::new("adb").arg("start-server");
    // }

    // print!("{:#?}", lines);
}

// 跳过登录时的公告栏
// fn skip_bulletin_board(adb: &ADB, device: &Device){
//     let point = Point{x:1,y:2};
//     adb.tap(device, point);

//     // 截图判断是否进入到登录界面
//     let current_page = adb.screenshot(device);
//     // check_login_page(current_page)?;

//     // Ok(());
// }

// // 验证是否在登录界面
// fn check_login_page() -> bool{

//     true
// }

/////////////////////////////////////////
// 行动力 AP
// fn auto_get_ap(){
// 判断当前时间大于11:00后
// 领取第一个体力补给,
// 判断当前时间大于17:00后
// 自动领取第二个专属补给
// 体力是直接加到 体力值上面的
// 判断条件：
// 第一次领取为 false 且时间大于11点,则领取第一次
// 第二次领取为 false 且时间大于15点,则领取第二次
// }
