#![warn(clippy::pedantic)]
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
// All the Champions in the game
pub enum Champions {
    Androxus,
    Ash,
    Atlas,
    Barik,
    BKing,
    Buck,
    Cassie,
    Corvus,
    Dredge,
    Drogoz,
    Evie,
    Fernando,
    Furia,
    Grohk,
    Grover,
    Imani,
    Inara,
    Io,
    Jenos,
    Khan,
    Kinessa,
    Koga,
    Lex,
    Lian,
    Maeve,
    Makoa,
    Damba,
    Moji,
    Octavia,
    Pip,
    Raum,
    Ruckus,
    Seris,
    ShaLin,
    Skye,
    Strix,
    Talus,
    Terminus,
    Tiberius,
    Torvald,
    Tyra,
    Viktor,
    Vivian,
    Vora,
    Willo,
    Yagorath,
    Ying,
    Zhin,
}

#[derive(PartialEq, Eq, Clone)]
// Classes each champion belongs to
pub enum Class {
    Damage,
    Flank,
    FrontLine,
    Support,
}

use Champions::*;
use Class::*;

impl Champions {
    // Returns the class that the champion belongs to
    pub fn get_class(&self) -> Class {
        match self {
            Ash | Atlas | Barik | Fernando | Inara | Khan | Makoa | Raum | Ruckus | Terminus
            | Torvald | Yagorath => FrontLine,
            BKing | Cassie | Dredge | Drogoz | Imani | Kinessa | Lian | Octavia | ShaLin
            | Strix | Tiberius | Tyra | Viktor | Vivian | Willo => Damage,
            Corvus | Furia | Grohk | Grover | Io | Jenos | Damba | Pip | Seris | Ying => Support,
            Androxus | Buck | Evie | Koga | Lex | Maeve | Moji | Skye | Talus | Vora | Zhin => {
                Flank
            }
        }
    }

    // Predicate that tests if the champion uses abilities frequently
    pub fn uses_abilities_frequently(&self) -> bool {
        match self {
            Viktor | Yagorath | Io | Strix => false,
            _ => true,
        }
    }

    // Predicate that tests if the champion's ult can be game-winning
    pub fn has_major_ult(&self) -> bool {
        match self {
            Viktor | Ying | Grohk | Pip | Imani => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion benefits from having a healer
    pub fn needs_healer(&self) -> bool {
        if self.get_class() == FrontLine {
            true
        } else {
            false
        }
    }

    // Predicate that tests if the champion has a high rate of fire
    pub fn has_sustained_fire(&self) -> bool {
        match self {
            Octavia | Tyra | Viktor | Vivian | Atlas | Barik | Fernando | Inara | Khan | Raum
            | Ruckus | Yagorath | Androxus | Buck | Koga | Lex | Moji | Skye | Talus | Vora
            | Corvus | Furia | Grohk | Jenos | Ying => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion uses explosives
    pub fn is_blaster(&self) -> bool {
        match self {
            BKing | Dredge | Drogoz | Willo | Ash | Evie | Pip => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion has a high DPS
    pub fn high_dps(&self) -> bool {
        if self.get_class() == Damage || self.get_class() == Flank {
            true
        } else {
            match self {
                Yagorath | Ying | Ruckus | Grohk => true,
                _ => false,
            }
        }
    }

    // Predicate that tests if the champion's shield is a standard shield
    pub fn has_normal_shield(&self) -> bool {
        match self {
            Vivian | Ash | Barik | Fernando | Khan | Makoa | Ruckus | Torvald => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion is slow and does not have a movement ability
    pub fn lacks_mobility(&self) -> bool {
        match self {
            Tyra | Terminus => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion frequently uses AOE attacks
    pub fn has_aoe(&self) -> bool {
        if self.is_blaster() {
            true
        } else {
            match self {
                Imani | Lian | Tiberius | Tyra | Viktor | Fernando | Inara | Terminus
                | Yagorath | Koga | Moji | Grohk | Seris => true,
                _ => false,
            }
        }
    }

    // Predicate that tests if the champion has a deployable
    pub fn has_deployable(&self) -> bool {
        match self {
            Imani | Vivian | Barik | Inara | Grohk | Io | Ying => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion's reload is slow or has some special
    // attack
    pub fn has_special_or_slow_reload(&self) -> bool {
        match self {
            Damba | Dredge | Pip => true,
            _ => false,
        }
    }

    // Predicate that tests if a champion has really low health (less than 2000)
    pub fn is_squishy(&self) -> bool {
        match self {
            Evie | Maeve => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion has a high base running speed
    pub fn is_fast(&self) -> bool {
        match self {
            Io | Talus => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion has a Crowd Control
    pub fn has_cc(&self) -> bool {
        match self {
            Strix | Willo | Ash | Atlas | Inara | Khan | Terminus | Buck | Evie | Maeve | Vora
            | Furia | Grohk | Grover | Io | Jenos | Damba | Pip => true,
            _ => false,
        }
    }

    // Predicate that tests if the champion can turn invisible
    pub fn has_cloak(&self) -> bool {
        match self {
            ShaLin | Strix | Skye | Seris => true,
            _ => false,
        }
    }
}
