//! The available magic the gathering formats.
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Format {
    Standard,
    Modern,
    Legacy,
    Vintage,
    Commander,
    Future,
    Pauper,
    Pioneer,
    Penny,
    Duel,
    OldSchool,
    Historic,
    Gladiator,
    Brawl,
    Premodern,
    PauperCommander,
    Alchemy,
    Explorer,
    Predh,
    Oathbreaker,
    Timeless,
    StandardBrawl,
    HistoricBrawl,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Format::*;
        write!(
            f,
            "{}",
            match self {
                Standard => "standard",
                Modern => "modern",
                Legacy => "legacy",
                Vintage => "vintage",
                Commander => "commander",
                Future => "future",
                Pauper => "pauper",
                Pioneer => "pioneer",
                Penny => "penny",
                Duel => "duel",
                OldSchool => "oldschool",
                Historic => "historic",
                Gladiator => "gladiator",
                Brawl => "brawl",
                Premodern => "premodern",
                PauperCommander => "paupercommander",
                Alchemy => "alchemy",
                Explorer => "explorer",
                Predh => "predh",
                Oathbreaker => "oathbreaker",
                Timeless => "timeless",
                StandardBrawl => "standardbrawl",
                HistoricBrawl => "historicbrawl",
            }
        )
    }
}
