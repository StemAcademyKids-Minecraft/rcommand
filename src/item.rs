use crate::{WORD, command};

pub enum Face {
    TOP = 0,
    BOTTOM = 1,
    XP = 3,
    XM = 2,
    YP = 4,
    YM = 5,
}

pub fn item(comm: WORD,x:i32,y:i32,z:i32,facing:Face) {
    
    const summon:&str = "summon minecraft:item_frame ";

    match comm {
        WORD::Sword => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_sword"),
        WORD::Acacia => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"acacia_wood"),
        WORD::Arrow => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"arrow"),
        WORD::Axe => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_axe"),
        WORD::Bed => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"red_bed"),
        WORD::Birch => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"birch_wood"),
        WORD::Boat =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"oak_boat"),
        WORD::Book =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"book"),
        WORD::Boots => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_boots"),
        WORD::Bow => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"bow"),
        WORD::Bucket => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"bucket"),
        WORD::Charcoal => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"charcoal"),
        WORD::Chest => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"chest"),
        WORD::Chestplate =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_chestplate"),
        WORD::Clock => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"clock"),
        WORD::Coal => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"coal"),
        WORD::Cobblestone => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"cobblestone"),
        WORD::Compass => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"compass"),
        WORD::CopperBlock =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"copper_block"),
        WORD::CopperIngot =>  command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"copper_ingot"),
        WORD::CopperOre => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"copper_ore"),
        WORD::CraftingTable => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"crafting_table"),
        WORD::DarkOak => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"dark_oak_wood"),
        WORD::Dirt => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"dirt"),
        WORD::FishingRod =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"fishing_rod"),
        WORD::Flint => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"flint"),
        WORD::FlintAndSteel =>command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"flint_and_steel"),
        WORD::Furnace => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"furnace"),
        WORD::GoldBlock => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"gold_block"),
        WORD::GoldIngot => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"gold_ingot"),
        WORD::GoldOre => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"gold_ore"),
        WORD::Helmet => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_helmet"),
        WORD::Hoe => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_hoe"),
        WORD::HorseArmor => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_horse_armor"),
        WORD::IronBlock => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_block"),
        WORD::IronOre => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_ore"),
        WORD::JungleTree => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"jungle_wood"),
        WORD::Ladder => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"ladder"),
        WORD::Lead => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"lead"),
        WORD::Leggings => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_leggings"),
        WORD::Mangrove =>  command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"mangrove_wood"),
        WORD::Map => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"map"),
        WORD::MusicDisc => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"music_disc_11"),
        WORD::NameTag => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"name_tag"),
        WORD::Paper => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"paper"),
        WORD::Pickaxe => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_pickaxe"),
        WORD::Plank => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"oak_planks"),
        WORD::RawCopper => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"raw_copper"),
        WORD::RawGold => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"raw_gold"),
        WORD::RawIron => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"raw_iron"),
        WORD::Saddle => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"saddle"),
        WORD::Sand => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"sand"),
        WORD::Shears => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"shears"),
        WORD::Shield => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"shield"),
        WORD::Shovel => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"iron_shovel"),
        WORD::Spruce => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"spruce_wood"),
        WORD::Spyglass => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"spyglass"),
        WORD::Stick => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"stick"),
        WORD::Stone => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"stone"),
        WORD::TotemOfUndying => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"totem_of_undying"),
        WORD::Wood => command!("{}~{} ~{} ~{} {{Facing:{},Invisible:1,Fixed:1b,Item:{{id:\"{}\",Count:1b}}}}",summon,x,y,z,facing as u32,"stripped_oak_wood"),
        _=> {}
    }
}