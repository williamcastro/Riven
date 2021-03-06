﻿///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use serde_repr::{ Serialize_repr, Deserialize_repr };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

/// League of Legends matchmaking seasons.
#[cfg_attr(feature = "nightly", non_exhaustive)]
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Serialize_repr, Deserialize_repr)]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Season {
    PRESEASON_3 = 0,
    SEASON_3 = 1,
    PRESEASON_2014 = 2,
    SEASON_2014 = 3,
    PRESEASON_2015 = 4,
    SEASON_2015 = 5,
    PRESEASON_2016 = 6,
    SEASON_2016 = 7,
    PRESEASON_2017 = 8,
    SEASON_2017 = 9,
    PRESEASON_2018 = 10,
    SEASON_2018 = 11,
    PRESEASON_2019 = 12,
    SEASON_2019 = 13,
    PRESEASON_2020 = 14,
    SEASON_2020 = 15,
}
