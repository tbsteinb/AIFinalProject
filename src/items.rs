#![warn(clippy::pedantic)]

enum Items {
    Illum,
    Res,
    Bshields,
    Haven,
    Nimble,
    Mriding,
    Mboost,
    Chronos,
    Rejuv,
    Veteran,
    Ktoheal,
    Lrip,
    Bulldozer,
    Caut,
    Dhands,
    Wrecker,
}

impl Items {
    fn get_base_cost(&self) -> usize {
        match self {
            Items::Illum
            | Items::Res
            | Items::Nimble
            | Items::Rejuv
            | Items::Veteran
            | Items::Bulldozer => 150,
            Items::Bshields | Items::Haven | Items::Chronos => 300,
            Items::Mriding | Items::Mboost | Items::Caut | Items::Dhands | Items::Wrecker => 250,
            Items::Ktoheal | Items::Lrip => 200,
        }
    }
}
