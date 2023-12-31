#[warn(unused_imports)]
use chrono::NaiveDate;

#[warn(unused_variables)]
pub struct UserInfo{
    uid:u32,
    name:String, // 用户名
    level: u8, // 等级
    // start_at: NaiveDate, // 不关心
    // gender:Gender,// 不关心
    // birthday: NaiveDate, // 不关心
    // team: String, // 不关心
    // team_positions:String, // 职位不关心
    sign: String,
}

// 性别
pub enum Gender {
    Famale,
    Male,
}

impl UserInfo{
    pub fn new(uid:u32, name:String, level:u8)-> Self{
        Self { uid, name, level, sign: String::new() }
    }

    // 修改个性签名
    pub fn edit_sign(self, sign:String){}


}