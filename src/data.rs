use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Mutex};

const ABILITY_CARDS_DATA: &str = include_str!("../worldhaven/data/character-ability-cards.js");
const ITEM_CARDS_DATA: &str = include_str!("../worldhaven/data/items.js");

static ABILITY_DATA: Mutex<Option<AbilityCards>> = Mutex::new(None);
static ITEM_DATA: Mutex<Option<ItemCards>> = Mutex::new(None);

pub fn get_data() -> Vec<Card> {
    let mut abilities = ABILITY_DATA.lock().unwrap();
    let mut items = ITEM_DATA.lock().unwrap();

    if abilities.is_none() {
        *abilities = Some(parse_data());
    }

    if items.is_none() {
        *items = Some(parse_items());
    }

    let abilities = abilities
        .as_ref()
        .unwrap()
        .iter()
        .map(|c| Card::Ability(c.clone()));

    let items = items
        .as_ref()
        .unwrap()
        .iter()
        .map(|c| Card::Item(c.clone()));

    abilities.chain(items).collect()
}

fn parse_data() -> AbilityCards {
    serde_json::from_str(ABILITY_CARDS_DATA).unwrap()
}

fn parse_items() -> ItemCards {
    serde_json::from_str(ITEM_CARDS_DATA).unwrap()
}

pub type AbilityCards = Vec<AbilityCard>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityCard {
    pub name: String,
    pub points: i32,
    pub expansion: String,
    pub image: String,
    pub xws: String,
    pub level: String,
    pub initiative: String,
    pub cardno: String,
}

pub type ItemCards = Vec<ItemCard>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemCard {
    pub name: String,
    pub points: i32,
    pub expansion: String,
    pub image: String,
    pub xws: String,
}

pub enum Card {
    Ability(AbilityCard),
    Item(ItemCard),
}

impl Card {
    fn name(&self) -> &str {
        match self {
            Card::Ability(c) => &c.name,
            Card::Item(c) => &c.name,
        }
    }

    fn expansion(&self) -> &str {
        match self {
            Card::Ability(c) => &c.expansion,
            Card::Item(c) => &c.expansion,
        }
    }

    pub fn image(&self) -> &str {
        match self {
            Card::Ability(c) => &c.image,
            Card::Item(c) => &c.image,
        }
    }

    pub fn is_playable_card(&self) -> bool {
        match self {
            Card::Ability(c) => c.level != "-",
            Card::Item(_) => true,
        }
    }
}

pub fn search_by_name(s: &str, allowed_classes: Option<&HashSet<ClassId>>) -> Option<Card> {
    let s = s.to_lowercase();
    let mut results = get_data();

    results.retain(Card::is_playable_card);

    results.retain(|c| c.expansion() == "Gloomhaven");

    for word in s.split_whitespace() {
        results.retain(|c| c.name().to_lowercase().contains(word));
    }

    if let Some(allowed_classes) = allowed_classes {
        results.retain(|c| {
            allowed_classes.iter().any(|class| {
                let s = "/".to_owned() + &get_class(class).abbreviation + "/";
                c.image().contains(&s)
            })
        });
    }

    results.into_iter().next()
}

pub fn get_classes() -> Classes {
    vec![
        Class::new(1, "Brute", "BR", "classIcons/01-Brute.png"),
        Class::new(2, "Tinkerer", "TI", "classIcons/02-Tinkerer.png"),
        Class::new(3, "Spellweaver", "SW", "classIcons/03-Spellweaver.png"),
        Class::new(4, "Scoundrel", "SC", "classIcons/04-Scoundrel.png"),
        Class::new(5, "Cragheart", "CH", "classIcons/05-Cragheart.png"),
        Class::new(6, "Mindthief", "MT", "classIcons/06-Mindthief.png"),
        Class::new(7, "Sun", "SK", "classIcons/07-Sun.png"),
        Class::new(8, "Three Spears", "QM", "classIcons/08-Three-Spears.png"),
        Class::new(9, "Circles", "SU", "classIcons/09-Circles.png"),
        Class::new(10, "Eclipse", "NS", "classIcons/10-Eclipse.png"),
        Class::new(11, "Squidface", "PH", "classIcons/11-Squidface.png"),
        Class::new(12, "Lightning", "BE", "classIcons/12-Lightning.png"),
        Class::new(13, "Music Note", "SS", "classIcons/13-MusicNote.png"),
        Class::new(14, "Angry Face", "DS", "classIcons/14-AngryFace.png"),
        Class::new(15, "Saw", "SB", "classIcons/15-Saw.png"),
        Class::new(16, "Triangles", "EL", "classIcons/16-Triangles.png"),
        Class::new(17, "Two Mini", "BT", "classIcons/17-Two-mini.png"),
    ]
}

pub fn get_class(id: &ClassId) -> Class {
    get_classes().iter().find(|c| &c.id == id).unwrap().clone()
}

pub type Classes = Vec<Class>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ClassId(i8);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Class {
    pub id: ClassId,
    pub name: String,
    pub abbreviation: String,
    pub icon_path: String,
}

impl Class {
    fn new(id: i8, name: &str, abbreviation: &str, icon_path: &str) -> Self {
        Class {
            id: ClassId(id),
            name: name.into(),
            abbreviation: abbreviation.into(),
            icon_path: icon_path.into(),
        }
    }
}
