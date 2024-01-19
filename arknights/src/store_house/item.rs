use crate::level::Level;

enum ItemCategory{
    /// 货币
    Currency,
    /// 升级经验
    UpgradeExperience,
    /// 升级技能
    UpgradeSkill,
    /// 其他还未定义的
    Other,
}

pub struct Item{
    /// 数量
    number: i32,
    /// 名称
    name: String,
    // drop_level: Option<Vec<Level>>,
}