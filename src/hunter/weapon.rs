// pub enum Type {
//     GreatSword,
//     LongSword,
//     SwordAndShield,
//     Hammer,
//     DualBlades,
//     SwitchAxe,
//     Lance,
//     GunLance,
//     HuntingHorn
// }

// impl Type {
//     pub fn as_str(&self) -> &'static str {
//         use Type::*;
//         match *self {
//             GreatSword => "Great Sword",
//             LongSword => "Long Sword",
//             SwordAndShield => "Sword and Shield",
//             Hammer => "Hammer",
//             _ => ""
//         }
//     }

//     pub fn from_str(string: &str) -> Type {
//         use Type::*;

//         match string {
//             "Great Sword" => GreatSword,
//             "Long Sword" => LongSword,
//             "Sword and Shield" =>SwordAndShield,
//             "Hammer" => Hammer,
//             _ => Lance

//         }
//     }
// }
