// 禁闭者管理

pub struct Hero{
    name:String,
    level:u8,
    quality:Quality,
    profession: Profession,
    // favorability_value: u8, // 好感度
    stars:u8,
}

pub enum Profession{
    坚韧,
    狂暴,
    诡秘,
    精准,
    异能,
    启迪,
}

pub enum Quality {
    狂,
    危,
    普,
}

// 更新所有禁必者信息

impl Hero {
    fn new(name:String,level:u8) -> Self{
        todo!()
    }
}

