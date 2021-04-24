#![warn(clippy::pedantic)]
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
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
pub enum Class {
    Damage,
    Flank,
    FrontLine,
    Support,
}

use Champions::*;
use Class::*;

impl Champions {
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

    pub fn uses_abilities_frequently(&self) -> bool {
        match self {
            Viktor | Yagorath | Io | Strix => false,
            _ => true,
        }
    }

    pub fn has_major_ult(&self) -> bool {
        match self {
            Viktor | Ying | Grohk | Pip | Imani => true,
            _ => false,
        }
    }

    pub fn needs_healer(&self) -> bool {
        if self.get_class() == FrontLine {
            true
        } else {
            false
        }
    }

    pub fn has_sustained_fire(&self) -> bool {
        match self {
            Octavia | Tyra | Viktor | Vivian | Atlas | Barik | Fernando | Inara | Khan | Raum
            | Ruckus | Yagorath | Androxus | Buck | Koga | Lex | Moji | Skye | Talus | Vora
            | Corvus | Furia | Grohk | Jenos | Ying => true,
            _ => false,
        }
    }

    pub fn is_blaster(&self) -> bool {
        match self {
            BKing | Dredge | Drogoz | Willo | Ash | Evie | Pip => true,
            _ => false,
        }
    }

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

    pub fn has_normal_shield(&self) -> bool {
        match self {
            Vivian | Ash | Barik | Fernando | Khan | Makoa | Ruckus | Torvald => true,
            _ => false,
        }
    }

    pub fn lacks_mobility(&self) -> bool {
        match self {
            Tyra | Terminus => true,
            _ => false,
        }
    }

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

    pub fn has_deployable(&self) -> bool {
        match self {
            Imani | Vivian | Barik | Inara | Grohk | Io | Ying => true,
            _ => false,
        }
    }

    pub fn has_special_or_slow_reload(&self) -> bool {
        match self {
            Damba | Dredge | Pip => true,
            _ => false,
        }
    }

    pub fn is_squishy(&self) -> bool {
        match self {
            Evie | Maeve => true,
            _ => false,
        }
    }

    pub fn is_fast(&self) -> bool {
        match self {
            Io | Talus => true,
            _ => false,
        }
    }

    pub fn has_cc(&self) -> bool {
        match self {
            Strix | Willo | Ash | Atlas | Inara | Khan | Terminus | Buck | Evie | Maeve | Vora
            | Furia | Grohk | Grover | Io | Jenos | Damba | Pip => true,
            _ => false,
        }
    }

    pub fn has_cloak(&self) -> bool {
        match self {
            ShaLin | Strix | Skye | Seris => true,
            _ => false,
        }
    }
}
