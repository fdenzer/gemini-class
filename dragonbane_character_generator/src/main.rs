// main.rs

use rand::Rng; // For later use in generation

// --- Enums ---

#[derive(Debug, Clone, PartialEq)]
pub enum Kin {
    Human,
    Halfling,
    Dwarf,
    Elf,
    Mallard,
    Wolfkin,
}

impl Kin {
    fn innate_abilities(&self) -> Vec<String> {
        match self {
            Kin::Human => vec!["Adaptive (You can choose to make a skill roll using another skill of your choice. Cost: 3 WP. GM has final say.)".to_string()],
            Kin::Halfling => vec!["Hard to Catch (Activate when dodging, get a boon to EVADE. Cost: 3 WP)".to_string()],
            Kin::Dwarf => vec!["(Dwarf innate ability not in pre-gens, needs to be found in full rules - Placeholder: Robust)".to_string()], // Placeholder
            Kin::Elf => vec!["Inner Peace (Meditate during stretch rest for D6 extra HP, D6 extra WP, and recover an additional condition. Unresponsive during meditation.)".to_string()],
            Kin::Mallard => vec!["Ill-tempered (Activate when making a skill roll for a boon. Become Angry. Cannot use for INT or INT-based skills. Cost: 3 WP)".to_string(), "Webbed Feet (Boon to SWIMMING rolls. Move at full speed in/under water.)".to_string()],
            Kin::Wolfkin => vec!["Hunting Instincts (Designate prey by sight/scent. Action in combat. Follow scent for a day. Spend 1 WP for boon on attack vs prey. Cost: 3 WP to designate)".to_string()],
        }
    }

    fn random() -> Self {
        match rand::thread_rng().gen_range(0..6) {
            0 => Kin::Human,
            1 => Kin::Halfling,
            2 => Kin::Dwarf,
            3 => Kin::Elf,
            4 => Kin::Mallard,
            _ => Kin::Wolfkin,
        }
    }
}

// --- Starting Gear, Coins, Memento Assignment ---
fn assign_starting_gear_coins_memento(
    profession: &Profession,
    inventory: &mut Vec<Item>,
    tiny_items: &mut Vec<Item>,
    weapons_at_hand: &mut Vec<Weapon>,
    armor: &mut Option<ArmorItem>,
    helmet: &mut Option<ArmorItem>,
    memento: &mut String,
    gold: &mut u32,
    silver: &mut u32,
    copper: &mut u32,
) {
    // Clear existing (if any, though typically these vecs/options would be empty when passed in)
    inventory.clear();
    tiny_items.clear();
    weapons_at_hand.clear();
    *armor = None;
    *helmet = None;

    match profession {
        Profession::MageElementalist => { // Archmaster Aodhan
            *memento = "Worn diary full of your experiences and discoveries.".to_string();
            *gold = 0; *silver = 7; *copper = 0; // From sheet (Aodhan has 7 silver)

            weapons_at_hand.push(Weapon {
                name: "Staff".to_string(),
                grip: "2h".to_string(),
                range: "2".to_string(), // Melee range
                damage: "D8".to_string(),
                durability: 9,
                features: vec!["Bludgeoning".to_string()],
                skill_used: "Staves".to_string(),
            });

            inventory.push(Item { name: "Spellbook".to_string(), quantity: 1, weight: 1, description: "Contains known spells and magical research.".to_string() });
            inventory.push(Item { name: "Torch".to_string(), quantity: 1, weight: 1, description: "Lights the way.".to_string() });
            // Armor: None for Aodhan
            // Helmet: None for Aodhan
            tiny_items.push(Item { name: "Flint & Tinder".to_string(), quantity: 1, weight: 0, description: "Starts fires.".to_string() });
        }
        Profession::Hunter => { // Orla Moonsilver
            *memento = "Fang from the troll that slew your sister.".to_string();
            *gold = 0; *silver = 6; *copper = 0; // Orla has 6 silver

            weapons_at_hand.push(Weapon {
                name: "Longbow".to_string(),
                grip: "2h".to_string(),
                range: "100".to_string(), // meters
                damage: "D12".to_string(),
                durability: 6, // PDF shows 6
                features: vec!["Piercing".to_string()],
                skill_used: "Bows".to_string(),
            });
            weapons_at_hand.push(Weapon {
                name: "Knife".to_string(), // She has one knife listed
                grip: "1h".to_string(),
                range: "1".to_string(), // Melee/Thrown - PDF indicates range 3 for thrown knives, but this is a melee knife
                damage: "D8".to_string(), // PDF shows D8 for her knife
                durability: 3, // PDF shows 3 for her knife
                features: vec!["Subtle".to_string(), "Piercing".to_string()],
                skill_used: "Knives".to_string(),
            });

            *armor = Some(ArmorItem { name: "Leather".to_string(), armor_rating: 1, bane_on: vec![] });
            // Helmet: None for Orla

            inventory.push(Item { name: "Quiver".to_string(), quantity: 1, weight: 1, description: "Holds arrows for the longbow.".to_string() }); // Assumed 20 arrows
            inventory.push(Item { name: "Torch".to_string(), quantity: 1, weight: 1, description: "Lights the way.".to_string() });
            inventory.push(Item { name: "Rope".to_string(), quantity: 1, weight: 1, description: "10 meters of rope.".to_string() });
            tiny_items.push(Item { name: "Flint & Tinder".to_string(), quantity: 1, weight: 0, description: "Starts fires.".to_string() });
        }
        Profession::Knight => { // Makander of Halfbay
            *memento = "A fine pipe made of black horn (a gift from your father).".to_string();
            *gold = 1; *silver = 0; *copper = 0; // Makander has 1 gold (10 silver)

            weapons_at_hand.push(Weapon {
                name: "Battleaxe".to_string(),
                grip: "1h".to_string(), // Can be 2h for -3 STR req, but listed as 1h here
                range: "2".to_string(),
                damage: "2D8".to_string(),
                durability: 9,
                features: vec!["Slashing".to_string()],
                skill_used: "Axes".to_string(),
            });
            weapons_at_hand.push(Weapon {
                name: "Short Sword".to_string(),
                grip: "1h".to_string(),
                range: "2".to_string(),
                damage: "D10".to_string(),
                durability: 12,
                features: vec!["Slashing".to_string(), "Piercing".to_string()],
                skill_used: "Swords".to_string(),
            });
            weapons_at_hand.push(Weapon { // Shield counts as a weapon at hand
                name: "Small Shield".to_string(),
                grip: "1h".to_string(),
                range: "N/A".to_string(), // Not used for attack range typically
                damage: "D8".to_string(), // Damage if used as improvised weapon? Or just its own block value. Pre-gen lists D8.
                durability: 15,
                features: vec!["Bludgeoning".to_string()], // If used to bash
                skill_used: "Swords".to_string(), // Or any STR based melee - Swords for Makander
            });

            *armor = Some(ArmorItem { name: "Plate".to_string(), armor_rating: 6, bane_on: vec!["Sneaking".to_string(), "Acrobatics".to_string(), "Evade".to_string()] });
            // Helmet: PDF says "Helmets can also give you a bane..." Makander's sheet doesn't explicitly list a separate helmet item, but plate usually implies one.
            // The pre-gen sheet for Makander has helmet rating integrated into the plate's stats/banes.
            // For simplicity, we'll assume plate includes helmet benefits unless specified otherwise.
            // His sheet shows Armor Rating 6 (Plate) and bane on Sneaking, Acro, Evade. This matches the plate description.
            // It does not explicitly list a separate helmet item providing additional AR or banes.

            inventory.push(Item { name: "Torch".to_string(), quantity: 1, weight: 1, description: "Lights the way.".to_string() });
            tiny_items.push(Item { name: "Flint & Tinder".to_string(), quantity: 1, weight: 0, description: "Starts fires.".to_string() });
        }
        Profession::Thief => { // Krisanna the Bold
            *memento = "A treasure map you \"found.\"".to_string();
            *gold = 0; *silver = 4; *copper = 0; // Krisanna has 4 silver

            // Krisanna has 3 daggers/knives listed in weapons.
            // Dagger 1h 8 D8 9 Subtle, Piercing, Slashing
            // Knife 1h 8 D8 6 Subtle, Piercing
            // Knife 1h 8 D8 6 Subtle, Piercing
            // The first one is "Dagger", others "Knife". Let's reflect that.
            weapons_at_hand.push(Weapon {
                name: "Dagger".to_string(),
                grip: "1h".to_string(),
                range: "8".to_string(), // Range 8m from sheet (likely throwing range)
                damage: "D8".to_string(),
                durability: 9,
                features: vec!["Subtle".to_string(), "Piercing".to_string(), "Slashing".to_string()],
                skill_used: "Knives".to_string(),
            });
            weapons_at_hand.push(Weapon {
                name: "Knife".to_string(),
                grip: "1h".to_string(),
                range: "8".to_string(),
                damage: "D8".to_string(),
                durability: 6,
                features: vec!["Subtle".to_string(), "Piercing".to_string()],
                skill_used: "Knives".to_string(),
            });
             weapons_at_hand.push(Weapon { // Third knife
                name: "Knife ".to_string(), // Adding a space to differentiate if needed, though names can be same
                grip: "1h".to_string(),
                range: "8".to_string(),
                damage: "D8".to_string(),
                durability: 6,
                features: vec!["Subtle".to_string(), "Piercing".to_string()],
                skill_used: "Knives".to_string(),
            });

            *armor = Some(ArmorItem { name: "Leather".to_string(), armor_rating: 1, bane_on: vec![] });
            // Helmet: None for Krisanna

            inventory.push(Item { name: "Lockpicks".to_string(), quantity: 1, weight: 1, description: "Set of lockpicks.".to_string() }); // Tiny item on sheet, but has weight 1 in some systems. PDF says tiny items are "hidden in a closed fist". Lockpicks might be slightly larger. For now, regular item.
                                                                                                                                          // Let's re-check PDF for lockpicks: "Picking a lock requires a SLEIGHT OF HAND roll. Doing so without lockpicks gives you a bane." (p12)
                                                                                                                                          // Krisanna's sheet lists Lockpicks in Inventory (not Tiny). So weight 1 is correct.
            inventory.push(Item { name: "Torch".to_string(), quantity: 1, weight: 1, description: "Lights the way.".to_string() });
            inventory.push(Item { name: "Rope".to_string(), quantity: 1, weight: 1, description: "10 meters of rope.".to_string() });
            tiny_items.push(Item { name: "Flint & Tinder".to_string(), quantity: 1, weight: 0, description: "Starts fires.".to_string() });
        }
        Profession::Fighter => { // Bastonn Bloodjaw
            *memento = "Blue bottle of perfume.".to_string();
            *gold = 0; *silver = 2; *copper = 0; // Bastonn has 2 silver

            weapons_at_hand.push(Weapon {
                name: "Long Spear".to_string(),
                grip: "2h".to_string(),
                range: "4".to_string(), // meters, Long feature
                damage: "2D8".to_string(),
                durability: 9,
                features: vec!["Long".to_string(), "Piercing".to_string()],
                skill_used: "Spears".to_string(),
            });
            weapons_at_hand.push(Weapon { // Second weapon is Short Spear
                name: "Short Spear".to_string(),
                grip: "1h".to_string(), // Can be thrown
                range: "36".to_string(), // Range from sheet (throwing range)
                damage: "D10".to_string(),
                durability: 9,
                features: vec!["Piercing".to_string()],
                skill_used: "Spears".to_string(),
            });

            *armor = Some(ArmorItem { name: "Studded Leather".to_string(), armor_rating: 2, bane_on: vec!["Sneaking".to_string()] });
            // Helmet: None for Bastonn, sheet shows bane on Sneaking for Studded Leather.

            inventory.push(Item { name: "Torch".to_string(), quantity: 1, weight: 1, description: "Lights the way.".to_string() });
            tiny_items.push(Item { name: "Flint & Tinder".to_string(), quantity: 1, weight: 0, description: "Starts fires.".to_string() });
        }
    }
}

// --- Derived Ratings Calculation ---
fn calculate_derived_ratings(attributes: &Attributes /*, kin: &Kin, profession: &Profession */) -> DerivedRatings {
    // HP = CON
    // WP = WIL
    // Movement & Damage Bonus: For now, placeholders or simple defaults.
    // These would ideally be influenced by kin/profession templates or more specific rules.

    // Placeholder values for Movement and Damage Bonus, as per plan
    let movement = 10; // Default placeholder
    let damage_bonus_str = match attributes.strength {
        0..=8 => "-".to_string(),
        9..=12 => "D2".to_string(), // Guessed progression, not from PDF
        13..=15 => "D4".to_string(),
        16..=17 => "D4".to_string(), // Makander STR 16 -> D4
        18.. => "D6".to_string(),   // Bastonn STR 18 -> D6
        // Orla STR 13 -> D4
        // Aodhan STR 8 -> -
        // Krisanna STR 8 -> -
    };
    let damage_bonus_agl = match attributes.agility {
        0..=8 => "-".to_string(),
        9..=12 => "D2".to_string(), // Guessed progression
        13..=15 => "D4".to_string(), // Bastonn AGL 14 -> D4
        16..=17 => "D6".to_string(), // Orla AGL 17 -> D6
        18.. => "D6".to_string(),   // Krisanna AGL 18 -> D6
        // Aodhan AGL 9 -> -
        // Makander AGL 10 -> -
    };


    // Let's use the pre-gen values as a lookup for now for Damage Bonus, as the pattern is not perfectly clear for a formula
    // This is still not ideal as it's based on specific pre-gen attribute scores.
    // A better approach would be a table from the full rules, or direct assignment by profession template.

    let db_str = match attributes.strength {
        13 | 14 | 15 | 16 | 17 => "D4".to_string(), // Orla (13), Makander (16)
        18 => "D6".to_string(),          // Bastonn (18)
        _ => "-".to_string(),            // Aodhan (8), Krisanna (8)
    };

    let db_agl = match attributes.agility {
        13 | 14 | 15 => "D4".to_string(), // Bastonn (14)
        16 | 17 | 18 => "D6".to_string(), // Orla (17), Krisanna (18)
        _ => "-".to_string(),            // Aodhan (9), Makander (10)
    };
    // The above matching is still a bit forced. The pre-gens are:
    // Aodhan: STR 8 (-), AGL 9 (-)
    // Orla: STR 13 (D4), AGL 17 (D6)
    // Makander: STR 16 (D4), AGL 10 (-)
    // Krisanna: STR 8 (-), AGL 18 (D6)
    // Bastonn: STR 18 (D6), AGL 14 (D4)

    // Simplified placeholder for now, will be refined with templates/rules
    let final_db_str = if attributes.strength >= 18 { "D6".to_string() }
                       else if attributes.strength >= 13 { "D4".to_string() }
                       else { "-".to_string() };

    let final_db_agl = if attributes.agility >= 16 { "D6".to_string() } // Covers 17, 18 from Orla, Krisanna
                       else if attributes.agility >= 13 { "D4".to_string() } // Covers 14 from Bastonn
                       else { "-".to_string() };


    DerivedRatings {
        hit_points_max: attributes.constitution,
        willpower_points_max: attributes.willpower,
        movement, // Placeholder
        damage_bonus_str: final_db_str, // Placeholder logic
        damage_bonus_agl: final_db_agl, // Placeholder logic
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Profession {
    MageElementalist, // Archmaster Aodhan
    Hunter,           // Orla Moonsilver
    Knight,           // Makander of Halfbay
    Thief,            // Krisanna the Bold
    Fighter,          // Bastonn Bloodjaw
    // Placeholder for other professions if we find them
}

impl Profession {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..5) {
            0 => Profession::MageElementalist,
            1 => Profession::Hunter,
            2 => Profession::Knight,
            3 => Profession::Thief,
            _ => Profession::Fighter,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Weakness {
    Arrogant,       // 1
    Greedy,         // 2
    Suspicious,     // 3
    ShortTempered,  // 4
    Stubborn,       // 5
    Vain,           // 6
    Cowardly,       // 7
    Jealous,        // 8
    Loudmouthed,    // 9
    Lazy,           // 10
    Fainthearted,   // 11 (Aodhan)
    Superstitious,  // 12
    Gullible,       // 13
    Bigoted,        // 14 (Orla)
    Reckless,       // 15 (Krisanna)
    Gluttonous,     // 16 (Bastonn)
    Foolhardy,      // 17 (Makander)
    Forgetful,      // 18
    Curious,        // 19
    // Option 20 is "Choose freely or roll again". For random generation, we'll just roll again (effectively D19).
}

impl Weakness {
    fn random() -> Self {
        match roll_d(19) { // D19 for the 19 specific weaknesses
            1 => Weakness::Arrogant,
            2 => Weakness::Greedy,
            3 => Weakness::Suspicious,
            4 => Weakness::ShortTempered,
            5 => Weakness::Stubborn,
            6 => Weakness::Vain,
            7 => Weakness::Cowardly,
            8 => Weakness::Jealous,
            9 => Weakness::Loudmouthed,
            10 => Weakness::Lazy,
            11 => Weakness::Fainthearted,
            12 => Weakness::Superstitious,
            13 => Weakness::Gullible,
            14 => Weakness::Bigoted,
            15 => Weakness::Reckless,
            16 => Weakness::Gluttonous,
            17 => Weakness::Foolhardy,
            18 => Weakness::Forgetful,
            _ => Weakness::Curious, // 19
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    Strength,
    Constitution,
    Agility,
    Intelligence,
    Willpower,
    Charisma,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillName {
    // Core Skills
    Acrobatics, Awareness, Bartering, BeastLore, Bluffing, Bushcraft, Crafting, Evade, Healing, HuntingAndFishing,
    Languages, MythsAndLegends, Performance, Persuasion, Riding, Seamanship, SleightOfHand, Sneaking, SpotHidden, Swimming,
    // Weapon Skills
    Axes, Bows, Brawling, Crossbows, Hammers, Knives, Slings, Spears, Staves, Swords,
    // Secondary Skills (Magic)
    Elementalism, // Animism, Mentalism (from PDF text, not pre-gens)
    // Placeholder for others
}

impl SkillName {
    fn get_base_attribute(&self) -> Attribute {
        match self {
            SkillName::Acrobatics => Attribute::Agility,
            SkillName::Awareness => Attribute::Intelligence,
            SkillName::Bartering => Attribute::Charisma,
            SkillName::BeastLore => Attribute::Intelligence,
            SkillName::Bluffing => Attribute::Charisma,
            SkillName::Bushcraft => Attribute::Intelligence,
            SkillName::Crafting => Attribute::Strength,
            SkillName::Evade => Attribute::Agility,
            SkillName::Healing => Attribute::Intelligence,
            SkillName::HuntingAndFishing => Attribute::Agility,
            SkillName::Languages => Attribute::Intelligence,
            SkillName::MythsAndLegends => Attribute::Intelligence,
            SkillName::Performance => Attribute::Charisma,
            SkillName::Persuasion => Attribute::Charisma,
            SkillName::Riding => Attribute::Agility,
            SkillName::Seamanship => Attribute::Intelligence,
            SkillName::SleightOfHand => Attribute::Agility,
            SkillName::Sneaking => Attribute::Agility,
            SkillName::SpotHidden => Attribute::Intelligence,
            SkillName::Swimming => Attribute::Agility, // Pre-gens show AGL for Swimming, though STR might also make sense. Sticking to pre-gens.

            SkillName::Axes => Attribute::Strength,
            SkillName::Bows => Attribute::Agility,
            SkillName::Brawling => Attribute::Strength,
            SkillName::Crossbows => Attribute::Agility,
            SkillName::Hammers => Attribute::Strength,
            SkillName::Knives => Attribute::Agility,
            SkillName::Slings => Attribute::Agility,
            SkillName::Spears => Attribute::Strength,
            SkillName::Staves => Attribute::Agility,
            SkillName::Swords => Attribute::Strength,

            SkillName::Elementalism => Attribute::Intelligence,
            // Define for Animism, Mentalism if added
        }
    }
}


#[derive(Debug, Clone)]
pub struct Skill {
    name: SkillName,
    description: String, // Optional: for more detailed info
    // base_attribute: Attribute, // Now derived from SkillName
    level: u8,
}

// --- Core Structs ---

#[derive(Debug, Clone)]
pub struct Attributes {
    strength: u8,
    constitution: u8,
    agility: u8,
    intelligence: u8,
    willpower: u8,
    charisma: u8,
}

#[derive(Debug, Clone)]
pub struct DerivedRatings {
    movement: u8,
    damage_bonus_str: String, // e.g., "D4", "-", "D6"
    damage_bonus_agl: String, // e.g., "D4", "-", "D6"
    hit_points_max: u8,
    willpower_points_max: u8,
}

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    quantity: u32,
    weight: u8, // 0 for tiny, 1 for normal, 2+ for heavy
    description: String,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    name: String,
    grip: String, // "1h", "2h"
    range: String, // "2m", "100m", etc. or specific value
    damage: String, // "D8", "2D6"
    durability: u8,
    features: Vec<String>, // "Subtle", "Piercing", "Long", "Bludgeoning"
    skill_used: String, // e.g., "Swords", "Bows"
}

#[derive(Debug, Clone)]
pub struct ArmorItem {
    name: String,
    armor_rating: u8,
    bane_on: Vec<String>, // e.g., "Sneaking", "Acrobatics"
}

#[derive(Debug, Clone)]
pub struct Character {
    name: String,
    kin: Kin,
    age: String, // "Old", "Adult", "Young"
    profession: Profession,
    appearance: String,
    weakness: Weakness,

    attributes: Attributes,
    derived_ratings: DerivedRatings,

    skills: Vec<Skill>, // All skills the character has
    // weapon_skills: Vec<Skill>, // Specifically weapon skills - or combine with general skills

    innate_abilities: Vec<String>, // Descriptions from Kin
    heroic_abilities: Vec<String>, // Or magic school/spells for mages
    spells: Vec<String>, // For mages

    inventory: Vec<Item>,
    tiny_items: Vec<Item>,
    weapons_at_hand: Vec<Weapon>,
    armor: Option<ArmorItem>,
    helmet: Option<ArmorItem>, // Helmets are also ArmorItems

    memento: String,
    gold: u32,
    silver: u32,
    copper: u32,

    // Runtime stats
    current_hp: u8,
    current_wp: u8,
    conditions: Vec<String>, // "Exhausted", "Sickly", etc.
}

// --- Placeholder for main function ---
fn main() {
    println!("Dragonbane Character Generator Initialized!");
    // Later, we will call character generation logic here
    // and then launch the Slint UI.

    let generated_attributes = generate_attributes();
    println!("Generated Attributes:");
    println!("STR: {}", generated_attributes.strength);
    println!("CON: {}", generated_attributes.constitution);
    println!("AGL: {}", generated_attributes.agility);
    println!("INT: {}", generated_attributes.intelligence);
    println!("WIL: {}", generated_attributes.willpower);
    println!("CHA: {}", generated_attributes.charisma);

    let selected_kin = Kin::random();
    println!("\nSelected Kin: {:?}", selected_kin);
    let kin_abilities = selected_kin.innate_abilities();
    println!("Innate Abilities: {:?}", kin_abilities);

    let selected_profession = Profession::random();
    println!("\nSelected Profession: {:?}", selected_profession);

    let selected_weakness = Weakness::random();
    println!("\nSelected Weakness: {:?}", selected_weakness);


    // Example instantiation (manual for now, will be generated later)
    // let example_attributes = Attributes {
    //     strength: 10,
    //     constitution: 12,
    //     agility: 14,
    //     intelligence: 13,
    //     willpower: 11,
    //     charisma: 15,
    // };

    // let example_derived = DerivedRatings {
    //     movement: 12, // Example, will be calculated
    //     damage_bonus_str: "-".to_string(),
    //     damage_bonus_agl: "D4".to_string(),
    //     hit_points_max: generated_attributes.constitution, // Use generated CON
    //     willpower_points_max: generated_attributes.willpower, // Use generated WIL
    // };
    let calculated_derived_ratings = calculate_derived_ratings(&generated_attributes);
    println!("\nCalculated Derived Ratings:");
    println!("HP Max: {}", calculated_derived_ratings.hit_points_max);
    println!("WP Max: {}", calculated_derived_ratings.willpower_points_max);
    println!("Movement: {}", calculated_derived_ratings.movement);
    println!("Damage Bonus STR: {}", calculated_derived_ratings.damage_bonus_str);
    println!("Damage Bonus AGL: {}", calculated_derived_ratings.damage_bonus_agl);

    let mut heroic_abilities = Vec::new();
    let mut spells = Vec::new();
    assign_profession_abilities_and_spells(&selected_profession, &mut heroic_abilities, &mut spells);
    println!("\nAssigned Heroic Abilities: {:?}", heroic_abilities);
    println!("Assigned Spells: {:?}", spells);

    let mut inventory = Vec::new();
    let mut tiny_items = Vec::new();
    let mut weapons_at_hand = Vec::new();
    let mut armor: Option<ArmorItem> = None;
    let mut helmet: Option<ArmorItem> = None;
    let mut memento = String::new();
    let mut gold = 0;
    let mut silver = 0;
    let mut copper = 0;

    assign_starting_gear_coins_memento(
        &selected_profession,
        &mut inventory,
        &mut tiny_items,
        &mut weapons_at_hand,
        &mut armor,
        &mut helmet,
        &mut memento,
        &mut gold,
        &mut silver,
        &mut copper,
    );
    println!("\nAssigned Memento: {}", memento);
    println!("Assigned Coins: G:{} S:{} C:{}", gold, silver, copper);
    println!("Assigned Weapons: {:?}", weapons_at_hand.iter().map(|w| &w.name).collect::<Vec<_>>());
    println!("Assigned Armor: {:?}", armor.as_ref().map(|a| &a.name));
    println!("Assigned Inventory: {:?}", inventory.iter().map(|i| &i.name).collect::<Vec<_>>());


    let example_character = Character {
        name: "Test Character".to_string(), // Will be randomized or input later
        kin: selected_kin,
        age: match selected_profession {
            Profession::MageElementalist => "Old".to_string(),
            Profession::Hunter | Profession::Knight => "Adult".to_string(),
            Profession::Thief | Profession::Fighter => "Young".to_string(),
        },
        profession: selected_profession,
        appearance: match selected_profession { // Using pre-gen appearances
            Profession::MageElementalist => "Tall and wiry. Long white beard and bushy eyebrows. Inquisitive eyes.".to_string(),
            Profession::Hunter => "Smooth and confident walk. Clear eyes that suspiciously scrutinize everyone. Eager and swift in thought and action.".to_string(),
            Profession::Knight => "Strong, sturdy, and stubborn. Waddling walk. Quick to anger when provoked, especially if someone insults your family or honor. You seldom smile.".to_string(),
            Profession::Thief => "Innocent face with shrewd, constantly observing eyes. Light and silent on your feet. You see opportunity in any situation.".to_string(),
            Profession::Fighter => "Scarred, bulging muscles. You look loyally at friends, but menacingly at foes. You take good care of your clothes, and often wear fragrant perfumes.".to_string(),
        },
        weakness: selected_weakness,
        attributes: generated_attributes,
        derived_ratings: calculated_derived_ratings,
        skills: assign_skills_for_profession(&selected_profession),
        innate_abilities: kin_abilities,
        heroic_abilities,
        spells,
        inventory,
        tiny_items,
        weapons_at_hand,
        armor,
        helmet,
        memento,
        gold,
        silver,
        copper,
        current_hp: calculated_derived_ratings.hit_points_max,
        current_wp: calculated_derived_ratings.willpower_points_max,
        conditions: vec![],
    };

    println!("\n--- Generated Character Summary ---");
    println!("Name: {}", example_character.name);
    println!("Kin: {:?}", example_character.kin);
    println!("Profession: {:?}", example_character.profession);
    println!("STR: {}", example_character.attributes.strength);
    println!("HP: {}/{}", example_character.current_hp, example_character.derived_ratings.hit_points_max);
    // println!("Assigned Skills:");
    // for skill in &example_character.skills {
    //     println!("- {:?} (Level {}) (Base: {:?})", skill.name, skill.level, skill.name.get_base_attribute());
    // }

    // --- Slint UI Setup ---
    let main_window = ui::CharacterWindow::new().unwrap();

    // Populate Slint CharacterData (converting types as needed)
    main_window.global::<ui::CharacterData>().set_name(example_character.name.into());
    main_window.global::<ui::CharacterData>().set_kin(format!("{:?}", example_character.kin).into());
    main_window.global::<ui::CharacterData>().set_profession(format!("{:?}", example_character.profession).into());
    main_window.global::<ui::CharacterData>().set_age(example_character.age.into());
    main_window.global::<ui::CharacterData>().set_appearance(example_character.appearance.into());
    main_window.global::<ui::CharacterData>().set_weakness(format!("{:?}", example_character.weakness).into());

    main_window.global::<ui::CharacterData>().set_str(example_character.attributes.strength as i32);
    main_window.global::<ui::CharacterData>().set_con(example_character.attributes.constitution as i32);
    main_window.global::<ui::CharacterData>().set_agl(example_character.attributes.agility as i32);
    main_window.global::<ui::CharacterData>().set_intel(example_character.attributes.intelligence as i32);
    main_window.global::<ui::CharacterData>().set_wil(example_character.attributes.willpower as i32);
    main_window.global::<ui::CharacterData>().set_cha(example_character.attributes.charisma as i32);

    main_window.global::<ui::CharacterData>().set_hp_current(example_character.current_hp as i32);
    main_window.global::<ui::CharacterData>().set_hp_max(example_character.derived_ratings.hit_points_max as i32);
    main_window.global::<ui::CharacterData>().set_wp_current(example_character.current_wp as i32);
    main_window.global::<ui::CharacterData>().set_wp_max(example_character.derived_ratings.willpower_points_max as i32);
    main_window.global::<ui::CharacterData>().set_movement(example_character.derived_ratings.movement as i32);
    main_window.global::<ui::CharacterData>().set_dmg_bonus_str(example_character.derived_ratings.damage_bonus_str.into());
    main_window.global::<ui::CharacterData>().set_dmg_bonus_agl(example_character.derived_ratings.damage_bonus_agl.into());

    // Convert Vec<String> to slint::ModelRc<slint::SharedString>
    let innate_abilities_slint: Vec<slint::SharedString> = example_character.innate_abilities.iter().map(|s| s.clone().into()).collect();
    main_window.global::<ui::CharacterData>().set_innate_abilities(slint::ModelRc::new(slint::VecModel::from(innate_abilities_slint)));

    let heroic_abilities_slint: Vec<slint::SharedString> = example_character.heroic_abilities.iter().map(|s| s.clone().into()).collect();
    main_window.global::<ui::CharacterData>().set_heroic_abilities(slint::ModelRc::new(slint::VecModel::from(heroic_abilities_slint)));

    let spells_slint: Vec<slint::SharedString> = example_character.spells.iter().map(|s| s.clone().into()).collect();
    main_window.global::<ui::CharacterData>().set_spells(slint::ModelRc::new(slint::VecModel::from(spells_slint)));

    // Convert skills
    let skills_slint_items: Vec<ui::skills_item> = example_character.skills.iter().map(|skill| {
        ui::skills_item {
            name: format!("{:?}", skill.name).into(),
            level: skill.level as i32,
            base_attr: format!("{:?}", skill.name.get_base_attribute()).into(),
        }
    }).collect();
    main_window.global::<ui::CharacterData>().set_skills(slint::ModelRc::new(slint::VecModel::from(skills_slint_items)));


    main_window.run().unwrap();
}

// --- Slint generated code integration ---
slint::include_modules!();

// --- Utility Functions ---
fn roll_d(sides: u8) -> u8 {
    if sides == 0 { return 0; } // Avoid panic on gen_range(1..=0)
    rand::thread_rng().gen_range(1..=sides)
}

fn roll_ndx(num_dice: u8, sides: u8) -> u8 {
    let mut total = 0;
    for _ in 0..num_dice {
        total += roll_d(sides);
    }
    total
}

// --- Attribute Generation ---
fn generate_attributes() -> Attributes {
    // Returns attributes rolled as 3d6
    Attributes {
        strength: roll_ndx(3, 6),
        constitution: roll_ndx(3, 6),
        agility: roll_ndx(3, 6),
        intelligence: roll_ndx(3, 6),
        willpower: roll_ndx(3, 6),
        charisma: roll_ndx(3, 6),
    }
}

// --- Skill Assignment ---
fn assign_skills_for_profession(profession: &Profession) -> Vec<Skill> {
    let mut skills = Vec::new();
    // Helper to add a skill
    let mut add_skill = |name: SkillName, level: u8| {
        skills.push(Skill { name, description: "".to_string(), level });
    };

    match profession {
        Profession::MageElementalist => {
            // Archmaster Aodhan's skills
            add_skill(SkillName::Awareness, 14);
            add_skill(SkillName::Bartering, 6);
            add_skill(SkillName::BeastLore, 14);
            add_skill(SkillName::Bushcraft, 6);
            add_skill(SkillName::Evade, 10);
            add_skill(SkillName::Healing, 14);
            add_skill(SkillName::Languages, 14); // Actually 5 on sheet, but INT is 16, skill is 14. Pre-gen likely has specific distribution.
            add_skill(SkillName::MythsAndLegends, 14); // Actually 6 on sheet.
            add_skill(SkillName::Persuasion, 6);
            add_skill(SkillName::SpotHidden, 12); // Actually 5 on sheet.
            // Weapon Skills
            add_skill(SkillName::Staves, 10); // Actually 5 on sheet.
            // Secondary Skills
            add_skill(SkillName::Elementalism, 14);

            // Correcting based on Aodhan's sheet values:
            skills.clear(); // Start fresh
            add_skill(SkillName::Acrobatics, 4);
            add_skill(SkillName::Awareness, 14);
            add_skill(SkillName::Bartering, 6);
            add_skill(SkillName::BeastLore, 14);
            add_skill(SkillName::Bluffing, 4); // Not listed, assuming low
            add_skill(SkillName::Bushcraft, 6);
            add_skill(SkillName::Crafting, 4); // Not listed, assuming low
            add_skill(SkillName::Evade, 10);
            add_skill(SkillName::Healing, 14);
            add_skill(SkillName::HuntingAndFishing, 4); // Not listed
            add_skill(SkillName::Languages, 5);
            add_skill(SkillName::MythsAndLegends, 6);
            add_skill(SkillName::Performance, 4); // Not listed
            add_skill(SkillName::Persuasion, 6);
            add_skill(SkillName::Riding, 4); // Not listed
            add_skill(SkillName::Seamanship, 4); // Not listed
            add_skill(SkillName::SleightOfHand, 4); // Not listed
            add_skill(SkillName::Sneaking, 4); // Not listed
            add_skill(SkillName::SpotHidden, 5);
            add_skill(SkillName::Swimming, 4); // Not listed
            // Weapon Skills
            add_skill(SkillName::Axes, 4); add_skill(SkillName::Bows, 5); add_skill(SkillName::Brawling, 4);
            add_skill(SkillName::Crossbows, 5); add_skill(SkillName::Hammers, 4); add_skill(SkillName::Knives, 5);
            add_skill(SkillName::Slings, 5); add_skill(SkillName::Spears, 4); add_skill(SkillName::Staves, 10);
            add_skill(SkillName::Swords, 4);
            // Secondary
            add_skill(SkillName::Elementalism, 14);
        }
        Profession::Hunter => {
            // Orla Moonsilver's skills
            skills.clear();
            add_skill(SkillName::Acrobatics, 14);
            add_skill(SkillName::Awareness, 12);
            add_skill(SkillName::Bartering, 5);
            add_skill(SkillName::BeastLore, 6);
            add_skill(SkillName::Bluffing, 5);
            add_skill(SkillName::Bushcraft, 12);
            add_skill(SkillName::Crafting, 6);
            add_skill(SkillName::Evade, 14);
            add_skill(SkillName::Healing, 6);
            add_skill(SkillName::HuntingAndFishing, 14);
            add_skill(SkillName::Languages, 6);
            add_skill(SkillName::MythsAndLegends, 6);
            add_skill(SkillName::Performance, 5);
            add_skill(SkillName::Persuasion, 5);
            add_skill(SkillName::Riding, 7);
            add_skill(SkillName::Seamanship, 6);
            add_skill(SkillName::SleightOfHand, 7);
            add_skill(SkillName::Sneaking, 14);
            add_skill(SkillName::SpotHidden, 6);
            add_skill(SkillName::Swimming, 14);
            // Weapon Skills
            add_skill(SkillName::Axes, 6); add_skill(SkillName::Bows, 14); add_skill(SkillName::Brawling, 6);
            add_skill(SkillName::Crossbows, 7); add_skill(SkillName::Hammers, 6); add_skill(SkillName::Knives, 14);
            add_skill(SkillName::Slings, 7); add_skill(SkillName::Spears, 6); add_skill(SkillName::Staves, 7);
            add_skill(SkillName::Swords, 12);
        }
        Profession::Knight => {
            // Makander of Halfbay's skills
            skills.clear();
            add_skill(SkillName::Acrobatics, 5); // AGL 10
            add_skill(SkillName::Awareness, 10); // INT 12
            add_skill(SkillName::Bartering, 6);
            add_skill(SkillName::BeastLore, 5);
            add_skill(SkillName::Bluffing, 6);
            add_skill(SkillName::Bushcraft, 5);
            add_skill(SkillName::Crafting, 7);
            add_skill(SkillName::Evade, 5); // AGL 10
            add_skill(SkillName::Healing, 5);
            add_skill(SkillName::HuntingAndFishing, 5);
            add_skill(SkillName::Languages, 5);
            add_skill(SkillName::MythsAndLegends, 10);
            add_skill(SkillName::Performance, 12);
            add_skill(SkillName::Persuasion, 12);
            add_skill(SkillName::Riding, 5);
            add_skill(SkillName::Seamanship, 5);
            add_skill(SkillName::SleightOfHand, 5);
            add_skill(SkillName::Sneaking, 5);
            add_skill(SkillName::SpotHidden, 5);
            add_skill(SkillName::Swimming, 5);
            // Weapon Skills
            add_skill(SkillName::Axes, 14); add_skill(SkillName::Bows, 5); add_skill(SkillName::Brawling, 14);
            add_skill(SkillName::Crossbows, 10); add_skill(SkillName::Hammers, 14); add_skill(SkillName::Knives, 5);
            add_skill(SkillName::Slings, 5); add_skill(SkillName::Spears, 14); add_skill(SkillName::Staves, 5);
            add_skill(SkillName::Swords, 14);
        }
        Profession::Thief => {
            // Krisanna the Bold's skills
            skills.clear();
            add_skill(SkillName::Acrobatics, 14); // AGL 18
            add_skill(SkillName::Awareness, 12); // INT 14
            add_skill(SkillName::Bartering, 5);
            add_skill(SkillName::BeastLore, 6);
            add_skill(SkillName::Bluffing, 10);
            add_skill(SkillName::Bushcraft, 6);
            add_skill(SkillName::Crafting, 4);
            add_skill(SkillName::Evade, 14); // AGL 18
            add_skill(SkillName::Healing, 6);
            add_skill(SkillName::HuntingAndFishing, 7);
            add_skill(SkillName::Languages, 6);
            add_skill(SkillName::MythsAndLegends, 6);
            add_skill(SkillName::Performance, 5);
            add_skill(SkillName::Persuasion, 5);
            add_skill(SkillName::Riding, 7);
            add_skill(SkillName::Seamanship, 6);
            add_skill(SkillName::SleightOfHand, 14); // AGL 18
            add_skill(SkillName::Sneaking, 14); // AGL 18
            add_skill(SkillName::SpotHidden, 12); // INT 14
            add_skill(SkillName::Swimming, 7);
            // Weapon Skills
            add_skill(SkillName::Axes, 4); add_skill(SkillName::Bows, 7); add_skill(SkillName::Brawling, 4);
            add_skill(SkillName::Crossbows, 7); add_skill(SkillName::Hammers, 4); add_skill(SkillName::Knives, 14); // AGL 18
            add_skill(SkillName::Slings, 7); add_skill(SkillName::Spears, 4); add_skill(SkillName::Staves, 7);
            add_skill(SkillName::Swords, 4);
        }
        Profession::Fighter => {
            // Bastonn Bloodjaw's skills
            skills.clear();
            add_skill(SkillName::Acrobatics, 12); // AGL 14
            add_skill(SkillName::Awareness, 5);  // INT 11
            add_skill(SkillName::Bartering, 4);
            add_skill(SkillName::BeastLore, 5);
            add_skill(SkillName::Bluffing, 4);
            add_skill(SkillName::Bushcraft, 5);
            add_skill(SkillName::Crafting, 7);
            add_skill(SkillName::Evade, 12); // AGL 14
            add_skill(SkillName::Healing, 5);
            add_skill(SkillName::HuntingAndFishing, 6);
            add_skill(SkillName::Languages, 5);
            add_skill(SkillName::MythsAndLegends, 5);
            add_skill(SkillName::Performance, 4);
            add_skill(SkillName::Persuasion, 4);
            add_skill(SkillName::Riding, 6);
            add_skill(SkillName::Seamanship, 5);
            add_skill(SkillName::SleightOfHand, 6);
            add_skill(SkillName::Sneaking, 12); // AGL 14
            add_skill(SkillName::SpotHidden, 5); // INT 11
            add_skill(SkillName::Swimming, 6);
            // Weapon Skills
            add_skill(SkillName::Axes, 14); add_skill(SkillName::Bows, 6); add_skill(SkillName::Brawling, 14);
            add_skill(SkillName::Crossbows, 6); add_skill(SkillName::Hammers, 14); add_skill(SkillName::Knives, 6);
            add_skill(SkillName::Slings, 6); add_skill(SkillName::Spears, 14); add_skill(SkillName::Staves, 6);
            add_skill(SkillName::Swords, 14);
        }
    }
    // Ensure all skills are present, even if at a base level (e.g. 4 or 5), if not set by profession.
    // For now, this function only returns skills explicitly defined by the pre-gen professions.
    // A more robust system would initialize all skills to a base and then allow profession/choices to increase them.
    skills
}

// --- Profession Abilities and Spells Assignment ---
fn assign_profession_abilities_and_spells(profession: &Profession, heroic_abilities: &mut Vec<String>, spells: &mut Vec<String>) {
    match profession {
        Profession::MageElementalist => {
            // No "Heroic Ability" per se, but has magic. The Elementalism skill is already assigned.
            // Spells from Archmaster Aodhan & PDF page 25
            spells.push("Fireball (Rank 1, Elementalist, Word/Gesture, Action, 20m, Instant. 2D6 damage. +1 PL = +D6 dmg or another fireball)".to_string());
            spells.push("Pillar (Rank 1, Elementalist, Word/Gesture, Action, 10m, Shift. Raises 3m high/1m wide pillar. +1 PL = +3m height)".to_string());
            spells.push("Gust of Wind (Rank 1, Elementalist, Word/Gesture, Action, 10m cone, Instant. Pushes 2D4m, 2D4 bludgeoning. Vs swarm 2D6. +1 PL = +1 die)".to_string());
            // Magic Tricks (Cost 1 WP, auto success, action)
            spells.push("Heat/Chill (Magic Trick: Area 10m becomes warm/cold. Protects vs cold for 1 shift)".to_string());
            spells.push("Puff of Smoke (Magic Trick: Impressive puff of smoke. Boon to SNEAKING in some situations)".to_string());
            spells.push("Ignite (Magic Trick: Light/extinguish candle/torch/lantern within 10m)".to_string());
        }
        Profession::Hunter => {
            heroic_abilities.push("Twin Shot (Cost 3 WP. When attacking with a bow, shoot two arrows. One attack roll with bane. Damage rolled separately. Can target same or different targets.)".to_string());
        }
        Profession::Knight => {
            heroic_abilities.push("Guardian (Cost 2 WP. If you and ally are within 2m of same enemy, and enemy attacks ally, force enemy to attack you instead. Out of turn, not an action.)".to_string());
        }
        Profession::Thief => {
            heroic_abilities.push("Backstabbing (Cost 3 WP. Melee attack vs enemy also within 2m of an ally. Counts as sneak attack: no dodge/parry, boon to roll, +1 damage die. Subtle weapon only. Not an action to activate.)".to_string());
        }
        Profession::Fighter => {
            heroic_abilities.push("Veteran (Cost 1 WP. Activate at start of combat round to retain initiative card from previous round. Not an action.)".to_string());
        }
    }
}


/*
    Full list of skills from pre-gen sheets:
    Acrobatics (AGL)
    Awareness (INT)
    Bartering (CHA)
    Beast Lore (INT)
    Bluffing (CHA)
    Bushcraft (INT)
    Crafting (STR)
    Evade (AGL)
    Healing (INT)
    Hunting & Fishing (AGL)
    Languages (INT)
    Myths & Legends (INT)
    Performance (CHA)
    Persuasion (CHA)
    Riding (AGL)
    Seamanship (INT)
    Sleight of Hand (AGL)
    Sneaking (AGL)
    Spot Hidden (INT)
    Swimming (AGL)

    Weapon Skills:
    Axes (STR)
    Bows (AGL)
    Brawling (STR)
    Crossbows (AGL)
    Hammers (STR)
    Knives (AGL)
    Slings (AGL)
    Spears (STR)
    Staves (AGL)
    Swords (STR)

    Secondary Skills (like magic):
    Elementalism (INT)
*/

/*
    Weaknesses from PDF Table (page 5):
    (Roll D20 or Choose)
    1 Arrogant
    2 Greedy
    3 Suspicious
    4 Short-tempered
    5 Stubborn
    6 Vain
    7 Cowardly
    8 Jealous
    9 Loudmouthed
    10 Lazy
    11 Fainthearted
    12 Superstitious
    13 Gullible
    14 Bigoted
    15 Reckless
    16 Gluttonous
    17 Foolhardy
    18 Forgetful
    19 Curious
    20 Choose freely or roll again
*/

```
