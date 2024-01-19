use crate::operator::Operator;

use self::control_center::ControlCenter;

mod control_center; // 控制中枢
mod dormitory; // 宿舍
mod generate_electricity_station; // 发电站
mod manufacture_station; // 制造站
mod office_room; // 办公室
mod process_station; // 加工站
mod reception_room; // 加工站
mod trading_post; // 贸易站
mod training_room; // 训练室

/// 基建
struct Infrastructure {
    control_center: ControlCenter,
    // rooms: Vec<dyn BaseRoom>
}

// 标定基建图片,获取所有设施信息
fn calib_infrastructure_room() {}

impl Infrastructure {
    fn new() {
        // adb 截图, 识别,定位,识别每个位置是什么设施,以及他的相信的信息
    }

    /// 设置基建副手
    fn set_deputy(&self, operators: Vec<Operator>) {
        // 按照 vec 的顺序, 会依次放入 控制中枢, B1, B2, B3, B4
    }
}

/// 设施类型
enum RoomType {
    /// 贸易站
    TradingPost,
    /// 制造站
    ManufactureStation,
    /// 发电站
    GenerateElectricityStation,
    /// 宿舍
    Dormitory,
    /// 会客室
    ReceptionRoom,
    /// 加工站
    ProcessStation,
    /// 办公室
    OfficeRoom,
    /// 训练室
    TrainingRoom,
    /// 控制中心
    ControlCenter,
}

trait BaseRoom {
    /// get 设施类型
    fn room_type(&self) -> RoomType;
    /// get 进驻人员信息
    fn operators(&self) -> Vec<Operator>;
    /// 升级设施
    fn upgrde(&self);

    /// get 进驻人员上限
    fn operator_max(&self) -> i32;
    /// 设置人员
    fn set_operator(&self);
    /// 清空人员
    fn clean_operator(&self);
}

struct RoomMetaData {
    level: i32,
    room_type: RoomType,
    operator_max: i32,
    operators: Vec<Operator>,
}

impl RoomMetaData {
    fn new(level: i32, room_type: RoomType, operator_max: i32, operators: Vec<Operator>) -> Self {
        Self {
            level,
            room_type,
            operator_max,
            operators,
        }
    }
}
