// 仓库

use self::item::Item;

mod item;




struct StoreHouse{
    items: Vec<Item>,
}

impl StoreHouse {
    fn add_items(&mut self, items: Vec<Item>){

    }

    fn get_item(&self, item_name: String){}
}