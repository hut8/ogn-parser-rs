//! APRS Symbol definitions and parsing
//!
//! Reference: https://www.aprs.org/symbols/symbolsX.txt

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AprsSymbol {
    // Primary Table (/)
    PoliceSheriff,          // /! BB
    ReservedRain,           // /" BC
    DigiWhiteCenter,        // /# BD
    Phone,                  // /$ BE
    DxCluster,              // /% BF
    HfGateway,              // /& BG
    SmallAircraft,          // /' BH
    MobileSatelliteStation, // /( BI
    WheelchairHandicapped,  // /) BJ
    Snowmobile,             // /* BK
    RedCross,               // /+ BL
    BoyScouts,              // /, BM
    HouseQthVhf,            // /- BN
    X,                      // /. BO
    RedDot,                 // // BP
    CircleObsolete,         // /0 P0
    TbdNumbered1,           // /1 P1
    TbdCirclesPool2,        // /2 P2
    TbdBalls3,              // /3 P3
    TbdOverlays4,           // /4 P4
    TbdNumbers5,            // /5 P5
    TbdAvailable6,          // /6 P6
    TbdNewUses7,            // /7 P7
    TbdMobilesEvents8,      // /8 P8
    TbdMobilesEvents9,      // /9 P9
    Fire,                   // /: MR
    CampgroundPortableOps,  // /; MS
    Motorcycle,             // /< MT
    RailroadEngine,         // /= MU
    Car,                    // /> MV
    ServerForFiles,         // /? MW
    HcFuturePredict,        // /@ MX
    AidStation,             // /A PA
    BbsOrPbbs,              // /B PB
    Canoe,                  // /C PC
    // /D PD (empty in reference)
    EyeballEvents,      // /E PE
    FarmVehicleTractor, // /F PF
    GridSquare6Digit,   // /G PG
    HotelBlueBed,       // /H PH
    TcpipOnAirNetwork,  // /I PI
    // /J PJ (empty in reference)
    School,                // /K PK
    PcUser,                // /L PL
    MacAprs,               // /M PM
    NtsStation,            // /N PN
    Balloon,               // /O PO
    Police,                // /P PP
    Tbd,                   // /Q PQ
    RecVehicle,            // /R PR
    Shuttle,               // /S PS
    Sstv,                  // /T PT
    Bus,                   // /U PU
    Atv,                   // /V PV
    NationalWxServiceSite, // /W PW
    Helo,                  // /X PX
    YachtSail,             // /Y PY
    WinAprs,               // /Z PZ
    HumanPerson,           // /[ HS
    TriangleDfStation,     // /\ HT
    MailPostOffice,        // /] HU
    LargeAircraft,         // /^ HV
    WeatherStation,        // /_ HW
    DishAntenna,           // /` HX
    Ambulance,             // /a LA
    Bike,                  // /b LB
    IncidentCommandPost,   // /c LC
    FireDept,              // /d LD
    HorseEquestrian,       // /e LE
    FireTruck,             // /f LF
    Glider,                // /g LG
    Hospital,              // /h LH
    IotaIslandsOnTheAir,   // /i LI
    Jeep,                  // /j LJ
    Truck,                 // /k LK
    Laptop,                // /l LL
    MicERepeater,          // /m LM
    NodeBlackBullseye,     // /n LN
    Eoc,                   // /o LO
    RoverPuppyDog,         // /p LP
    GridSqShownAbove128m,  // /q LQ
    Repeater,              // /r LR
    ShipPwrBoat,           // /s LS
    TruckStop,             // /t LT
    Truck18Wheeler,        // /u LU
    Van,                   // /v LV
    WaterStation,          // /w LW
    XAprsUnix,             // /x LX
    YagiAtQth,             // /y LY
    TbdZ,                  // /z LZ
    // /{ J1 (empty in reference)
    TncStreamSwitch1, // /| J2
    // /} J3 (empty in reference)
    TncStreamSwitch2, // /~ J4

    // Alternate Table (\)
    EmergencyAndOverlays,     // \! OBO
    ReservedAlt,              // \" OC
    OverlayDigiGreenStar,     // \# OD#
    BankAtmGreenBox,          // \$ OEO
    PowerPlantWithOverlay,    // \% OFO
    IgateRxTx1hop2hop,        // \& OG#
    CrashIncidentSites,       // \' OHO
    CloudyOtherClouds,        // \( OIO
    FirenetMeoModisEarthObs,  // \) OJO
    AvailSnowMoved,           // \* OK
    Church,                   // \+ OL
    GirlScouts,               // \, OM
    HouseHfOpPresent,         // \- ONO
    AmbiguousBigQuestionMark, // \. OO
    WaypointDestination,      // \/ OP
    CircleIrlpEcholinkWires,  // \0 A0#
    Avail1,                   // \1 A1
    Avail2,                   // \2 A2
    Avail3,                   // \3 A3
    Avail4,                   // \4 A4
    Avail5,                   // \5 A5
    Avail6,                   // \6 A6
    Avail7,                   // \7 A7
    Network80211OrOther,      // \8 A8O
    GasStationBluePump,       // \9 A9
    AvailHail,                // \: NR
    ParkPicnicOverlayEvents,  // \; NSO
    AdvisoryOneWxFlag,        // \< NTO
    AvailSymbolOverlayGroup,  // \= NUO
    OverlayedCarsVehicles,    // \> NV#
    InfoKioskBlueBox,         // \? NW
    HuricanetropStorm,        // \@ NX
    OverlayBoxDtmfRfidXo,     // \A AA#
    AvailBlowingSnow,         // \B AB
    CoastGuard,               // \C AC
    DepotsAndDrizzle,         // \D ADO
    SmokeAndOtherVisCodes,    // \E AE
    AvailFreezingRain,        // \F AF
    AvailSnowShower,          // \G AG
    HazeAndOverlayHazards,    // \H AHO
    RainShower,               // \I AI
    AvailLightning,           // \J AJ
    KenwoodHt,                // \K AK
    Lighthouse,               // \L AL
    MarsArmyNavyAf,           // \M AMO
    NavigationBuoy,           // \N AN
    OverlayBalloonRocket,     // \O AO
    Parking,                  // \P AP
    Quake,                    // \Q AQ
    Restaurant,               // \R ARO
    SatellitePacsat,          // \S AS
    Thunderstorm,             // \T AT
    Sunny,                    // \U AU
    VortacNavAid,             // \V AV
    NwsSiteWithOptions,       // \W AW#
    PharmacyRxApothicary,     // \X AX
    RadiosAndDevices,         // \Y AYO
    AvailZ,                   // \Z AZ
    WCloudAndHumansOverlay,   // \[ DSO
    NewOverlayableGpsSymbol,  // \\ DTO
    AvailBackslash,           // \] DU
    OtherAircraftOverlays,    // \^ DV#
    WxSiteGreenDigi,          // \_ DW#
    RainAllTypesWithOverlay,  // \` DX
    ArrlAresWinlinkDstar,     // \a SA#O
    AvailBlowingDustSand,     // \b SB
    CdTriangleRacesSatern,    // \c SC#O
    DxSpotByCallsign,         // \d SD
    SleetAndFutureOverlays,   // \e SE
    FunnelCloud,              // \f SF
    GaleFlags,                // \g SG
    StoreOrHamfest,           // \h SHO
    BoxOrPointsOfInterest,    // \i SI#
    WorkZoneSteamShovel,      // \j SJ
    SpecialVehicleSuvAtv4x4,  // \k SKO
    AreasBoxCircles,          // \l SL
    ValueSign3DigitDisplay,   // \m SM
    OverlayTriangle,          // \n SN#
    SmallCircle,              // \o SO
    AvailPartlyCloudy,        // \p SP
    AvailQ,                   // \q SQ
    Restrooms,                // \r SR
    OverlayShipBoats,         // \s SS#
    Tornado,                  // \t ST
    OverlayedTruck,           // \u SU#
    OverlayedVan,             // \v SV#
    FloodingAvalanchesSlides, // \w SWO
    WreckOrObstruction,       // \x SX
    Skywarn,                  // \y SY
    OverlayedShelter,         // \z SZ#
    AvailFog,                 // \{ Q1
    TncStreamSwitchAlt1,      // \| Q2
    AvailMaybe,               // \} Q3
    TncStreamSwitchAlt2,      // \~ Q4
}

impl AprsSymbol {
    /// Parse APRS symbol from table and symbol characters
    ///
    /// # Arguments
    /// * `table` - Symbol table character ('/' for primary, '\' for alternate)
    /// * `symbol` - Symbol character
    ///
    /// # Example
    /// ```
    /// use ogn_parser::AprsSymbol;
    ///
    /// let symbol = AprsSymbol::parse('/', '_');
    /// assert_eq!(symbol, Some(AprsSymbol::WeatherStation));
    /// ```
    pub fn parse(table: char, symbol: char) -> Option<Self> {
        match (table, symbol) {
            // Primary table (/)
            ('/', '!') => Some(Self::PoliceSheriff),
            ('/', '"') => Some(Self::ReservedRain),
            ('/', '#') => Some(Self::DigiWhiteCenter),
            ('/', '$') => Some(Self::Phone),
            ('/', '%') => Some(Self::DxCluster),
            ('/', '&') => Some(Self::HfGateway),
            ('/', '\'') => Some(Self::SmallAircraft),
            ('/', '(') => Some(Self::MobileSatelliteStation),
            ('/', ')') => Some(Self::WheelchairHandicapped),
            ('/', '*') => Some(Self::Snowmobile),
            ('/', '+') => Some(Self::RedCross),
            ('/', ',') => Some(Self::BoyScouts),
            ('/', '-') => Some(Self::HouseQthVhf),
            ('/', '.') => Some(Self::X),
            ('/', '/') => Some(Self::RedDot),
            ('/', '0') => Some(Self::CircleObsolete),
            ('/', '1') => Some(Self::TbdNumbered1),
            ('/', '2') => Some(Self::TbdCirclesPool2),
            ('/', '3') => Some(Self::TbdBalls3),
            ('/', '4') => Some(Self::TbdOverlays4),
            ('/', '5') => Some(Self::TbdNumbers5),
            ('/', '6') => Some(Self::TbdAvailable6),
            ('/', '7') => Some(Self::TbdNewUses7),
            ('/', '8') => Some(Self::TbdMobilesEvents8),
            ('/', '9') => Some(Self::TbdMobilesEvents9),
            ('/', ':') => Some(Self::Fire),
            ('/', ';') => Some(Self::CampgroundPortableOps),
            ('/', '<') => Some(Self::Motorcycle),
            ('/', '=') => Some(Self::RailroadEngine),
            ('/', '>') => Some(Self::Car),
            ('/', '?') => Some(Self::ServerForFiles),
            ('/', '@') => Some(Self::HcFuturePredict),
            ('/', 'A') => Some(Self::AidStation),
            ('/', 'B') => Some(Self::BbsOrPbbs),
            ('/', 'C') => Some(Self::Canoe),
            ('/', 'D') => None, // Empty in reference
            ('/', 'E') => Some(Self::EyeballEvents),
            ('/', 'F') => Some(Self::FarmVehicleTractor),
            ('/', 'G') => Some(Self::GridSquare6Digit),
            ('/', 'H') => Some(Self::HotelBlueBed),
            ('/', 'I') => Some(Self::TcpipOnAirNetwork),
            ('/', 'J') => None, // Empty in reference
            ('/', 'K') => Some(Self::School),
            ('/', 'L') => Some(Self::PcUser),
            ('/', 'M') => Some(Self::MacAprs),
            ('/', 'N') => Some(Self::NtsStation),
            ('/', 'O') => Some(Self::Balloon),
            ('/', 'P') => Some(Self::Police),
            ('/', 'Q') => Some(Self::Tbd),
            ('/', 'R') => Some(Self::RecVehicle),
            ('/', 'S') => Some(Self::Shuttle),
            ('/', 'T') => Some(Self::Sstv),
            ('/', 'U') => Some(Self::Bus),
            ('/', 'V') => Some(Self::Atv),
            ('/', 'W') => Some(Self::NationalWxServiceSite),
            ('/', 'X') => Some(Self::Helo),
            ('/', 'Y') => Some(Self::YachtSail),
            ('/', 'Z') => Some(Self::WinAprs),
            ('/', '[') => Some(Self::HumanPerson),
            ('/', '\\') => Some(Self::TriangleDfStation),
            ('/', ']') => Some(Self::MailPostOffice),
            ('/', '^') => Some(Self::LargeAircraft),
            ('/', '_') => Some(Self::WeatherStation),
            ('/', '`') => Some(Self::DishAntenna),
            ('/', 'a') => Some(Self::Ambulance),
            ('/', 'b') => Some(Self::Bike),
            ('/', 'c') => Some(Self::IncidentCommandPost),
            ('/', 'd') => Some(Self::FireDept),
            ('/', 'e') => Some(Self::HorseEquestrian),
            ('/', 'f') => Some(Self::FireTruck),
            ('/', 'g') => Some(Self::Glider),
            ('/', 'h') => Some(Self::Hospital),
            ('/', 'i') => Some(Self::IotaIslandsOnTheAir),
            ('/', 'j') => Some(Self::Jeep),
            ('/', 'k') => Some(Self::Truck),
            ('/', 'l') => Some(Self::Laptop),
            ('/', 'm') => Some(Self::MicERepeater),
            ('/', 'n') => Some(Self::NodeBlackBullseye),
            ('/', 'o') => Some(Self::Eoc),
            ('/', 'p') => Some(Self::RoverPuppyDog),
            ('/', 'q') => Some(Self::GridSqShownAbove128m),
            ('/', 'r') => Some(Self::Repeater),
            ('/', 's') => Some(Self::ShipPwrBoat),
            ('/', 't') => Some(Self::TruckStop),
            ('/', 'u') => Some(Self::Truck18Wheeler),
            ('/', 'v') => Some(Self::Van),
            ('/', 'w') => Some(Self::WaterStation),
            ('/', 'x') => Some(Self::XAprsUnix),
            ('/', 'y') => Some(Self::YagiAtQth),
            ('/', 'z') => Some(Self::TbdZ),
            ('/', '{') => None, // Empty in reference
            ('/', '|') => Some(Self::TncStreamSwitch1),
            ('/', '}') => None, // Empty in reference
            ('/', '~') => Some(Self::TncStreamSwitch2),

            // Alternate table (\)
            ('\\', '!') => Some(Self::EmergencyAndOverlays),
            ('\\', '"') => Some(Self::ReservedAlt),
            ('\\', '#') => Some(Self::OverlayDigiGreenStar),
            ('\\', '$') => Some(Self::BankAtmGreenBox),
            ('\\', '%') => Some(Self::PowerPlantWithOverlay),
            ('\\', '&') => Some(Self::IgateRxTx1hop2hop),
            ('\\', '\'') => Some(Self::CrashIncidentSites),
            ('\\', '(') => Some(Self::CloudyOtherClouds),
            ('\\', ')') => Some(Self::FirenetMeoModisEarthObs),
            ('\\', '*') => Some(Self::AvailSnowMoved),
            ('\\', '+') => Some(Self::Church),
            ('\\', ',') => Some(Self::GirlScouts),
            ('\\', '-') => Some(Self::HouseHfOpPresent),
            ('\\', '.') => Some(Self::AmbiguousBigQuestionMark),
            ('\\', '/') => Some(Self::WaypointDestination),
            ('\\', '0') => Some(Self::CircleIrlpEcholinkWires),
            ('\\', '1') => Some(Self::Avail1),
            ('\\', '2') => Some(Self::Avail2),
            ('\\', '3') => Some(Self::Avail3),
            ('\\', '4') => Some(Self::Avail4),
            ('\\', '5') => Some(Self::Avail5),
            ('\\', '6') => Some(Self::Avail6),
            ('\\', '7') => Some(Self::Avail7),
            ('\\', '8') => Some(Self::Network80211OrOther),
            ('\\', '9') => Some(Self::GasStationBluePump),
            ('\\', ':') => Some(Self::AvailHail),
            ('\\', ';') => Some(Self::ParkPicnicOverlayEvents),
            ('\\', '<') => Some(Self::AdvisoryOneWxFlag),
            ('\\', '=') => Some(Self::AvailSymbolOverlayGroup),
            ('\\', '>') => Some(Self::OverlayedCarsVehicles),
            ('\\', '?') => Some(Self::InfoKioskBlueBox),
            ('\\', '@') => Some(Self::HuricanetropStorm),
            ('\\', 'A') => Some(Self::OverlayBoxDtmfRfidXo),
            ('\\', 'B') => Some(Self::AvailBlowingSnow),
            ('\\', 'C') => Some(Self::CoastGuard),
            ('\\', 'D') => Some(Self::DepotsAndDrizzle),
            ('\\', 'E') => Some(Self::SmokeAndOtherVisCodes),
            ('\\', 'F') => Some(Self::AvailFreezingRain),
            ('\\', 'G') => Some(Self::AvailSnowShower),
            ('\\', 'H') => Some(Self::HazeAndOverlayHazards),
            ('\\', 'I') => Some(Self::RainShower),
            ('\\', 'J') => Some(Self::AvailLightning),
            ('\\', 'K') => Some(Self::KenwoodHt),
            ('\\', 'L') => Some(Self::Lighthouse),
            ('\\', 'M') => Some(Self::MarsArmyNavyAf),
            ('\\', 'N') => Some(Self::NavigationBuoy),
            ('\\', 'O') => Some(Self::OverlayBalloonRocket),
            ('\\', 'P') => Some(Self::Parking),
            ('\\', 'Q') => Some(Self::Quake),
            ('\\', 'R') => Some(Self::Restaurant),
            ('\\', 'S') => Some(Self::SatellitePacsat),
            ('\\', 'T') => Some(Self::Thunderstorm),
            ('\\', 'U') => Some(Self::Sunny),
            ('\\', 'V') => Some(Self::VortacNavAid),
            ('\\', 'W') => Some(Self::NwsSiteWithOptions),
            ('\\', 'X') => Some(Self::PharmacyRxApothicary),
            ('\\', 'Y') => Some(Self::RadiosAndDevices),
            ('\\', 'Z') => Some(Self::AvailZ),
            ('\\', '[') => Some(Self::WCloudAndHumansOverlay),
            ('\\', '\\') => Some(Self::NewOverlayableGpsSymbol),
            ('\\', ']') => Some(Self::AvailBackslash),
            ('\\', '^') => Some(Self::OtherAircraftOverlays),
            ('\\', '_') => Some(Self::WxSiteGreenDigi),
            ('\\', '`') => Some(Self::RainAllTypesWithOverlay),
            ('\\', 'a') => Some(Self::ArrlAresWinlinkDstar),
            ('\\', 'b') => Some(Self::AvailBlowingDustSand),
            ('\\', 'c') => Some(Self::CdTriangleRacesSatern),
            ('\\', 'd') => Some(Self::DxSpotByCallsign),
            ('\\', 'e') => Some(Self::SleetAndFutureOverlays),
            ('\\', 'f') => Some(Self::FunnelCloud),
            ('\\', 'g') => Some(Self::GaleFlags),
            ('\\', 'h') => Some(Self::StoreOrHamfest),
            ('\\', 'i') => Some(Self::BoxOrPointsOfInterest),
            ('\\', 'j') => Some(Self::WorkZoneSteamShovel),
            ('\\', 'k') => Some(Self::SpecialVehicleSuvAtv4x4),
            ('\\', 'l') => Some(Self::AreasBoxCircles),
            ('\\', 'm') => Some(Self::ValueSign3DigitDisplay),
            ('\\', 'n') => Some(Self::OverlayTriangle),
            ('\\', 'o') => Some(Self::SmallCircle),
            ('\\', 'p') => Some(Self::AvailPartlyCloudy),
            ('\\', 'q') => Some(Self::AvailQ),
            ('\\', 'r') => Some(Self::Restrooms),
            ('\\', 's') => Some(Self::OverlayShipBoats),
            ('\\', 't') => Some(Self::Tornado),
            ('\\', 'u') => Some(Self::OverlayedTruck),
            ('\\', 'v') => Some(Self::OverlayedVan),
            ('\\', 'w') => Some(Self::FloodingAvalanchesSlides),
            ('\\', 'x') => Some(Self::WreckOrObstruction),
            ('\\', 'y') => Some(Self::Skywarn),
            ('\\', 'z') => Some(Self::OverlayedShelter),
            ('\\', '{') => Some(Self::AvailFog),
            ('\\', '|') => Some(Self::TncStreamSwitchAlt1),
            ('\\', '}') => Some(Self::AvailMaybe),
            ('\\', '~') => Some(Self::TncStreamSwitchAlt2),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_station_symbol() {
        let symbol = AprsSymbol::parse('/', '_');
        assert_eq!(symbol, Some(AprsSymbol::WeatherStation));
    }

    #[test]
    fn test_emergency_symbol() {
        let symbol = AprsSymbol::parse('\\', '!');
        assert_eq!(symbol, Some(AprsSymbol::EmergencyAndOverlays));
    }

    #[test]
    fn test_invalid_symbol() {
        let symbol = AprsSymbol::parse('/', '\x7f');
        assert_eq!(symbol, None);
    }

    #[test]
    fn test_invalid_table() {
        let symbol = AprsSymbol::parse('?', '_');
        assert_eq!(symbol, None);
    }
}
