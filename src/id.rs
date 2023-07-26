use crate::{sound::WORD, command, item::Face};

pub struct MineCarftId<'a>{
    id:&'a str,
}



impl MineCarftId<'_> {
    const SUMMON:&str = "summon";


    pub fn get(word:WORD,) -> Self {

        Self { id: Self::convert(word),}
    }

    pub fn item(&self,xyz:(isize,isize,isize)) {
        command!("{} item ~{} ~{} ~{} {{Item:{{id:\"minecraft:{}\",Count:1b}}}}",Self::SUMMON,xyz.0,xyz.1,xyz.2,self.id);
    }

    pub fn frame(&self,xyz:(isize,isize,isize),facing:Face) {
        command!("{} item_frame ~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",Self::SUMMON,xyz.0,xyz.1,xyz.2,facing as u32,self.id);
    }

    pub fn mob(&self) {}



    fn convert<'a>(word:WORD) -> &'a str {
        match word {
            WORD::Sword => "iron_sword",
            WORD::Acacia => "acacia_wood",
            WORD::Arrow => "arrow",
            WORD::Axe => "iron_axe",
            WORD::Bed => "red_bed",
            WORD::Birch => "birch_wood",
            WORD::Boat =>"oak_boat",
            WORD::Book =>"book",
            WORD::Boots => "iron_boots",
            WORD::Bow => "bow",
            WORD::Bucket => "bucket",
            WORD::Charcoal => "charcoal",
            WORD::Chest => "chest",
            WORD::Chestplate =>"iron_chestplate",
            WORD::Clock => "clock",
            WORD::Coal => "coal",
            WORD::Cobblestone => "cobblestone",
            WORD::Compass => "compass",
            WORD::CopperBlock =>"copper_block",
            WORD::CopperIngot =>  "copper_ingot",
            WORD::CopperOre => "copper_ore",
            WORD::CraftingTable => "crafting_table",
            WORD::DarkOak => "dark_oak_wood",
            WORD::Dirt => "dirt",
            WORD::FishingRod =>"fishing_rod",
            WORD::Flint => "flint",
            WORD::FlintAndSteel =>"flint_and_steel",
            WORD::Furnace => "furnace",
            WORD::GoldBlock => "gold_block",
            WORD::GoldIngot => "gold_ingot",
            WORD::GoldOre => "gold_ore",
            WORD::Helmet => "iron_helmet",
            WORD::Hoe => "iron_hoe",
            WORD::HorseArmor => "iron_horse_armor",
            WORD::IronBlock => "iron_block",
            WORD::IronOre => "iron_ore",
            WORD::JungleTree => "jungle_wood",
            WORD::Ladder => "ladder",
            WORD::Lead => "lead",
            WORD::Leggings => "iron_leggings",
            WORD::Mangrove =>  "mangrove_wood",
            WORD::Map => "map",
            WORD::MusicDisc => "music_disc_11",
            WORD::NameTag => "name_tag",
            WORD::Paper => "paper",
            WORD::Pickaxe => "iron_pickaxe",
            WORD::Plank => "oak_planks",
            WORD::RawCopper => "raw_copper",
            WORD::RawGold => "raw_gold",
            WORD::RawIron => "raw_iron",
            WORD::Saddle => "saddle",
            WORD::Sand => "sand",
            WORD::Shears => "shears",
            WORD::Shield => "shield",
            WORD::Shovel => "iron_shovel",
            WORD::Spruce => "spruce_wood",
            WORD::Spyglass => "spyglass",
            WORD::Stick => "stick",
            WORD::Stone => "stone",
            WORD::TotemOfUndying => "totem_of_undying",
            WORD::Wood => "stripped_oak_wood",
            _=> "a"
        }
    }



}