#![warn(clippy::pedantic)]

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
