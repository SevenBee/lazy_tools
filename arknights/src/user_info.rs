// TODO: 添加常量，理智最大值 = 999

use crate::operator::Operator;

pub struct UserInfo{
    uid:i32,    // 用户id
    level: i32, // 用户等级
    sanity: Sanity, // 理智
    operators: Vec<Operator>,
}

// 理智值
pub struct Sanity{
    usable_value: i32, // 剩余理智
    max_value:i32 // 理智上限
}