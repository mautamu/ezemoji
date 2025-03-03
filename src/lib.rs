//! EZEmojis is a Work in progress made for [rust-rain](https://rusty-rain.xyz) program
//! so just a warning that the api may change a lot in the coming updates.
//!
//! If you have any suggestion feel free to open a issue on the
//! [github page](https://github.com/cowboy8625/ezemoji)
use std::collections::HashMap;
use std::hash::Hash;

/// All Default Implemented Emojis Groups.
///
/// Some of these may not show up for you depending on your Font I think.
/// correct me if I am wrong.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EmojiGroups {
    Emojis,
    Japanese,
    Smile,
    Moon,
    Earth,
    Plant,
    Clock,
    Shape,
    Arrow,
    HorizontalDominos,
    VerticalDominos,
    Cards,
    NumberedBalls,
    NumberedCubes,
    LargeLetter,
    Crab,
    All,
}

/// CharGroups are a collection of u32 numbers that group like characters together.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CharGroups<T: Hash + Eq> {
    /// Default Implemented Groups
    Standard(EmojiGroups),
    /// Place your Custom Groups in this one.
    Custom(T),
}

impl<T: Hash + Eq> From<EmojiGroups> for CharGroups<T> {
    fn from(v: EmojiGroups) -> Self {
        Self::Standard(v)
    }
}

impl<T: Hash + Eq> CharGroups<T> {
    pub fn custom(v: T) -> Self {
        Self::Custom(v)
    }
}

/// Creates A `HashMap<EmojiGroups, Vec<u32>>` and returns it.
pub fn create_emoji_data<T: Hash + Eq>() -> HashMap<CharGroups<T>, Vec<u32>> {
    let mut hash = HashMap::new();
    hash.insert(EmojiGroups::Emojis.into(), create_emojis());
    // FIXME:
    hash.insert(EmojiGroups::Smile.into(), create_smile());

    hash.insert(EmojiGroups::Japanese.into(), create_jap());
    hash.insert(EmojiGroups::Moon.into(), create_moon());
    hash.insert(EmojiGroups::Earth.into(), create_earth());
    hash.insert(EmojiGroups::Plant.into(), create_plant());
    hash.insert(EmojiGroups::Clock.into(), create_clock());
    hash.insert(EmojiGroups::Shape.into(), create_shape());
    hash.insert(EmojiGroups::Arrow.into(), create_arrows());
    hash.insert(EmojiGroups::HorizontalDominos.into(), create_hdominos());
    hash.insert(EmojiGroups::VerticalDominos.into(), create_vdominos());
    hash.insert(EmojiGroups::Cards.into(), create_cards());
    hash.insert(
        EmojiGroups::NumberedBalls.into(),
        create_letter_ball_solid(),
    );
    hash.insert(
        EmojiGroups::NumberedCubes.into(),
        create_letter_cube_solid(),
    );
    hash.insert(EmojiGroups::LargeLetter.into(), create_bold_large_letters());
    hash.insert(EmojiGroups::Crab.into(), create_crab());
    hash.insert(EmojiGroups::All.into(), create_all());
    hash
}

pub struct EZEmojis<T: Hash + Eq> {
    list: HashMap<CharGroups<T>, Vec<u32>>,
}

impl<T: Hash + Eq> Default for EZEmojis<T> {
    fn default() -> Self {
        Self {
            list: create_emoji_data(),
        }
    }
}

impl<T: Hash + Eq> EZEmojis<T> {
    pub fn add(&mut self, key: CharGroups<T>, value: Vec<u32>) {
        self.list.insert(key, value);
    }

    pub fn get_char(&self, key: &CharGroups<T>) -> Option<Vec<char>> {
        self.list.get(key).map(|n| {
            n.iter()
                .map(|num| std::char::from_u32(*num).unwrap_or(' '))
                .collect()
        })
    }

    pub fn get_u32(&self, key: &CharGroups<T>) -> Option<&Vec<u32>> {
        self.list.get(key)
    }
}

pub fn create_emojis() -> Vec<u32> {
    let mut e: Vec<_> = (129292..=129400).collect();
    e.append(&mut (129402..=129482).collect());
    e.append(&mut (129484..=129535).collect());
    e
}

pub fn create_smile() -> Vec<u32> {
    (128512..=128518).collect()
}

pub fn create_jap() -> Vec<u32> {
    (65382..=65437).collect()
}

pub fn create_moon() -> Vec<u32> {
    (127760..=127773).collect()
}

pub fn create_earth() -> Vec<u32> {
    (127757..=127760).collect()
}

pub fn create_plant() -> Vec<u32> {
    (127793..=127827).collect()
}

pub fn create_clock() -> Vec<u32> {
    (128336..=128359).collect()
}

pub fn create_shape() -> Vec<u32> {
    (128992..=129003).collect()
}

pub fn create_crab() -> Vec<u32> {
    vec![129408]
}

pub fn create_arrows() -> Vec<u32> {
    let mut a: Vec<u32> = (129024..=129035).collect();
    a.append(&mut (129040..=129095).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129104..=129113).collect());
    a
}

pub fn create_all() -> Vec<u32> {
    let mut a = create_moon();
    a.append(&mut create_earth());
    a.append(&mut create_plant());
    a.append(&mut create_clock());
    a.append(&mut create_shape());
    a.append(&mut create_arrows());
    a.append(&mut create_hdominos());
    a.append(&mut create_vdominos());
    a.append(&mut create_cards());
    a.append(&mut create_crab());
    a.append(&mut create_letter_ball_solid());
    a.append(&mut create_letter_cube_solid());
    a.append(&mut create_bold_large_letters());
    a
}

pub fn create_hdominos() -> Vec<u32> {
    (127024..=127073).collect()
}

pub fn create_vdominos() -> Vec<u32> {
    (127074..=127123).collect()
}

pub fn create_cards_hearts() -> Vec<u32> {
    (127137..=127166).collect()
}

pub fn create_cards_diamonds() -> Vec<u32> {
    (127169..=127182).collect()
}

pub fn create_cards_clubs() -> Vec<u32> {
    (127185..=127198).collect()
}

pub fn create_cards() -> Vec<u32> {
    let mut h = create_cards_hearts();
    h.append(&mut create_cards_clubs());
    h.append(&mut create_cards_diamonds());
    h
}

pub fn create_letter_ball_solid() -> Vec<u32> {
    (127312..=127337).collect()
}

pub fn create_letter_cube_solid() -> Vec<u32> {
    (127344..=127369).collect()
}

pub fn create_bold_large_letters() -> Vec<u32> {
    (127462..=127487).collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_emojis() {}
}
