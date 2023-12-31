// 干员
pub struct Operator{
    // 等级
    level:i32,
    // 职业
    profession: Profession,
    
    // 稀有度
    rarity: Rarity,
    // tag: Vec<Tag>
    name: String,
    advancements:i32,
    potential:i32,
}

// pub enum Tag{
//     "治疗",'支援','输出','群攻','减速','生存','防护','削弱','位移','控场',有好多没写完
// }

pub enum Rarity {
    F = 1,
    E = 2,
    D = 3,
    C = 4,
    B = 5,
    A = 6
}

// 职业
pub enum Profession {
    先锋,
    近卫,
    狙击,
    重装,
    医疗,
    辅助,
    术士,
    特种
}