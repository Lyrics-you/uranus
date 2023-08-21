use std::{cell::RefCell, rc::Rc};

use crate::{
    components::{Charism, Delete, Replace},
    utils::join_path,
};

pub struct Assemble<'a> {
    pub always_fishing_point_charism: Rc<RefCell<Charism<'a>>>,
    pub catch_better_fish_charism: Rc<RefCell<Charism<'a>>>,
    pub easier_to_pick_up_charism: Rc<RefCell<Charism<'a>>>,
    pub gifit_trait_quick_upgrade_charism: Rc<RefCell<Charism<'a>>>,
    pub free_store_exchange_charism: Rc<RefCell<Charism<'a>>>,
    pub always_hero_raity_trait_charism: Rc<RefCell<Charism<'a>>>,
    pub hades_path: String,
}

impl<'a> Assemble<'a> {
    pub fn new() -> Self {
        Assemble {
            always_fishing_point_charism: Rc::new(RefCell::new(Charism::new(
                "Fishing",
                "Always Fishing Point",
                "Always eligible to fish.",
            ))),
            catch_better_fish_charism: Rc::new(RefCell::new(Charism::new(
                "Fishing",
                "Catch Better Fish",
                "Increase biomefish weight.",
            ))),
            easier_to_pick_up_charism: Rc::new(RefCell::new(Charism::new(
                "Fishing",
                "Easier To Pick Up",
                "Increase fishing success.",
            ))),
            gifit_trait_quick_upgrade_charism: Rc::new(RefCell::new(Charism::new(
                "GifitTrait",
                "GifitTrait Quick Upgrade",
                "Change chamber thresholds to one.",
            ))),
            free_store_exchange_charism: Rc::new(RefCell::new(Charism::new(
                "FreeStore",
                "Free Store Exchange",
                "Modify the broker cost amount to negative.",
            ))),
            always_hero_raity_trait_charism: Rc::new(RefCell::new(Charism::new(
                "HeroRarity",
                "Always Hero Raity Trait",
                "Always hero raity trait.",
            ))),
            hades_path: "".to_string(),
        }
    }

    pub fn set_hades_path(&mut self, hades_path: String) {
        self.hades_path = hades_path;
    }

    pub fn assemble_always_fishing_point(&self) {
        let binding = self.always_fishing_point_charism.clone();
        let always_fishing_point_charism = binding.borrow();
        always_fishing_point_charism.add(Delete::new(
            join_path(&self.hades_path, "RoomManager.lua"),
            r"and IsFishingEligible\( currentRun, currentRoom \)",
        ));
    }

    pub fn assemble_catch_better_fish(&self) {
        let binding = self.catch_better_fish_charism.clone();
        let catch_better_fish_charism = binding.borrow();
        catch_better_fish_charism.add(Replace::new(
            join_path(&self.hades_path, "FishingData.lua"),
            r"Weight = 0.05",
            "Weight = 10",
        ));
    }

    pub fn assemble_easier_to_pick_up(&self) {
        let binding = self.easier_to_pick_up_charism.clone();
        let easier_to_pick_up_charism = binding.borrow();
        easier_to_pick_up_charism
            .add(Replace::new(
                join_path(&self.hades_path, "FishingData.lua"),
                r"NumFakeDunks.*?\},",
                "NumFakeDunks = { Min = 0, Max = 0 },",
            ))
            .add(Replace::new(
                join_path(&self.hades_path, "FishingData.lua"),
                r"GoodInterval.*?,",
                "GoodInterval = 3,",
            ))
            .add(Replace::new(
                join_path(&self.hades_path, "FishingData.lua"),
                r"PerfectInterval.*?,",
                "PerfectInterval = 1,",
            ));
    }

    pub fn assemble_gifit_trait_quick_upgrade(&self) {
        let binding = self.gifit_trait_quick_upgrade_charism.clone();
        let gifit_trait_quick_upgrade_charism = binding.borrow();
        gifit_trait_quick_upgrade_charism.add(Replace::new(
            join_path(&self.hades_path, "TraitData.lua"),
            r"ChamberThresholds.*?\},",
            "ChamberThresholds =  { 1, 1 },",
        ));
    }

    pub fn assemble_free_store_exchange(&self) {
        let binding = self.free_store_exchange_charism.clone();
        let free_store_exchange_charism = binding.borrow();
        free_store_exchange_charism.add(Replace::new(
            join_path(&self.hades_path, "StoreData.lua"),
            r"CostAmount = ",
            "CostAmount = -",
        ));
    }

    pub fn assemble_always_hero_raity_trait(&self) {
        // elseif rarityTable.Epic[upgradeData.ItemName] and lootData.RarityChances.Heroic and RandomChance( lootData.RarityChances.Heroic) then
        // =>
        // elseif rarityTable.Heroic[upgradeData.ItemName] then

        // if validRarities.Legendary and lootData.RarityChances.Legendary and RandomChance( lootData.RarityChances.Legendary )
        // =>
        // if validRarities.Legendary

        // elseif validRarities.Heroic and lootData.RarityChances.Heroic and RandomChance( lootData.RarityChances.Heroic )
        // =>
        // elseif validRarities.Heroic

        // r#"if validRarities.Rare and lootData.RarityChances.Rare then
        // 	chosenRarity = "Rare"
        // 	chosenUpgrade = GetRandomValue( rarityTable.Rare )"#;
        // =>
        // r#"if validRarities.Heroic  then
        // 	chosenRarity = "Heroic"
        // 	chosenUpgrade = GetRandomValue( rarityTable.Heroic )"#;

        // r#"elseif validRarities.Heroic and lootData.RarityChances.Heroic then
        // 	chosenRarity = "Heroic"
        // 	chosenUpgrade = GetRandomValue( rarityTable.Heroic )"#;
        // =>
        // r#"elseif validRarities.Rare and lootData.RarityChances.Rare then
        // 	chosenRarity = "Rare"
        // 	chosenUpgrade = GetRandomValue( rarityTable.Rare )"#;

        let binding = self.always_hero_raity_trait_charism.clone();
        let always_hero_raity_trait_charism = binding.borrow();

        always_hero_raity_trait_charism
        .add(Replace::new(
            join_path(&self.hades_path,"TraitScripts.lua"), 
             r"elseif rarityTable.Legendary.*?then",
             "elseif rarityTable.Heroic[upgradeData.ItemName] then"
            ))
        .add(Replace::new(
            join_path(&self.hades_path, "TraitScripts.lua"),
            r"if validRarities.Legendary and lootData.RarityChances.Legendary and RandomChance\( lootData.RarityChances.Legendary \)",
            "if validRarities.Legendary"
        ))
        .add(Replace::new(
            join_path(&self.hades_path, "TraitScripts.lua"),
            r"elseif validRarities.Heroic and lootData.RarityChances.Heroic and RandomChance\( lootData.RarityChances.Heroic \)",
            "elseif validRarities.Heroic"
        ))
        .add(Replace::new(
            join_path(&self.hades_path, "TraitScripts.lua"),
            // r"if validRarities.Rare and lootData.RarityChances.Rare then\n.*?\n.*?Rare \)",
            r"\tif validRarities.Rare.*?\n.*?\n.*?\)",
            r#"        if validRarities.Heroic  then
            chosenRarity = "Heroic"
            chosenUpgrade = GetRandomValue( rarityTable.Heroic )"#,
        ))
        .add(Replace::new(
            join_path(&self.hades_path, "TraitScripts.lua"),
            r#"elseif validRarities.Heroic.*?\n.*?\n.*?\)"#,
            r#"elseif validRarities.Rare and lootData.RarityChances.Rare then
            chosenRarity = "Rare"
            chosenUpgrade = GetRandomValue( rarityTable.Rare )"#,
        ));
    }

    pub fn assemble_all(&self) {
        self.assemble_always_fishing_point();
        self.assemble_catch_better_fish();
        self.assemble_easier_to_pick_up();
        self.assemble_gifit_trait_quick_upgrade();
        self.assemble_free_store_exchange();
        self.assemble_always_hero_raity_trait();
    }
}

impl Default for Assemble<'_> {
    fn default() -> Self {
        Assemble::new()
    }
}
