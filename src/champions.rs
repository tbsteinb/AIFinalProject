#![warn(clippy::pedantic)]

#[derive(PartialEq, Eq, Clone)]
enum Champions {
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
enum Class {
    Damage,
    Flank,
    FrontLine,
    Support,
}

impl Champions {
    fn get_class(&self) -> Class {
        match self {
            Champions::Ash
            | Champions::Atlas
            | Champions::Barik
            | Champions::Fernando
            | Champions::Inara
            | Champions::Khan
            | Champions::Makoa
            | Champions::Raum
            | Champions::Ruckus
            | Champions::Terminus
            | Champions::Torvald
            | Champions::Yagorath => Class::FrontLine,
            Champions::BKing
            | Champions::Cassie
            | Champions::Dredge
            | Champions::Drogoz
            | Champions::Imani
            | Champions::Kinessa
            | Champions::Lian
            | Champions::Octavia
            | Champions::ShaLin
            | Champions::Strix
            | Champions::Tiberius
            | Champions::Tyra
            | Champions::Viktor
            | Champions::Vivian
            | Champions::Willo => Class::Damage,
            Champions::Corvus
            | Champions::Furia
            | Champions::Grohk
            | Champions::Grover
            | Champions::Io
            | Champions::Jenos
            | Champions::Damba
            | Champions::Pip
            | Champions::Seris
            | Champions::Ying => Class::Support,
            Champions::Androxus
            | Champions::Buck
            | Champions::Evie
            | Champions::Koga
            | Champions::Lex
            | Champions::Maeve
            | Champions::Moji
            | Champions::Skye
            | Champions::Talus
            | Champions::Vora
            | Champions::Zhin => Class::Flank,
        }
    }
}

impl Champions {
    fn uses_abilities_frequently(&self) -> bool {
        match self {
            Champions::Viktor | Champions::Yagorath | Champions::Io | Champions::Strix => false,
            _ => true,
        }
    }

    fn has_major_ult(&self) -> bool {
        match self {
            Champions::Viktor
            | Champions::Ying
            | Champions::Grohk
            | Champions::Corvus
            | Champions::Pip
            | Champions::Imani => true,
            _ => false,
        }
    }

    fn needs_healer(&self) -> bool {
        if self.get_class() == Class::FrontLine {
            true
        } else {
            false
        }
    }

    fn has_sustained_fire(&self) -> bool {
        unimplemented!()
    }

    fn is_blaster(&self) -> bool {
        unimplemented!()
    }

    fn high_dps(&self) -> bool {
        if self.get_class() == Class::Damage || self.get_class() == Class::Flank {
            true
        } else {
            false
        }
    }

    fn has_normal_shield(&self) -> bool {
        unimplemented!()
    }

    fn lacks_mobility(&self) -> bool {
        match self {
            Champions::Tyra | Champions::Terminus => true,
            _ => false,
        }
    }

    fn has_aoe(&self) -> bool {
        unimplemented!()
    }

    fn has_deployable(&self) -> bool {
        unimplemented!()
    }

    fn has_special_or_slow_reload(&self) -> bool {
        match self {
            Champions::Damba | Champions::Dredge | Champions::Pip => true,
            _ => false,
        }
    }

    fn is_squishy(&self) -> bool {
        match self {
            Champions::Evie | Champions::Maeve => true,
            _ => false,
        }
    }

    fn is_fast(&self) -> bool {
        match self {
            Champions::Io | Champions::Talus => true,
            _ => false,
        }
    }
}
