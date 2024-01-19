use crate::operator::Operator;

use super::{RoomMetaData, RoomType};

/// 控制中心
pub struct ControlCenter{
    meta_data:RoomMetaData,
}

impl ControlCenter {
    fn new(level:i32, operator_max:i32, operators: Vec<Operator>) -> Self{
        let room_type = RoomType::ControlCenter;
        let meta_data = RoomMetaData::new(level, room_type, operator_max, operators);
        Self { meta_data }
    }
}

