use std::fmt::Write;
use std::str::FromStr;

use flat_projection::FlatPoint;
use flat_projection::FlatProjection;
use serde::Serialize;

use crate::AprsError;
use crate::EncodeError;
use crate::Timestamp;
use crate::lonlat::{Latitude, Longitude, encode_latitude, encode_longitude};
use crate::position_comment::PositionComment;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct AprsPosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,
    pub messaging_supported: bool,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub symbol_table: char,
    pub symbol_code: char,
    #[serde(flatten)]
    pub comment: PositionComment,

    #[serde(skip_serializing)]
    pub flat_projection: FlatProjection<f64>,
    #[serde(skip_serializing)]
    pub flat_point: FlatPoint<f64>,
}

impl FromStr for AprsPosition {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let messaging_supported = s.starts_with('=') || s.starts_with('@');
        let has_timestamp = s.starts_with('@') || s.starts_with('/');

        // check for minimal message length
        if (!has_timestamp && s.len() < 19) || (has_timestamp && s.len() < 26) {
            return Err(AprsError::InvalidPosition(s.to_owned()));
        };

        // Extract timestamp and remaining string
        let (timestamp, s) = if has_timestamp {
            (Some(s[1..8].parse()?), &s[8..])
        } else {
            (None, &s[1..])
        };

        // check for compressed position format
        let is_uncompressed_position = s.chars().take(1).all(|c| c.is_numeric());
        if !is_uncompressed_position {
            return Err(AprsError::UnsupportedPositionFormat(s.to_owned()));
        }

        // parse position
        let mut latitude: Latitude = s[0..8].parse()?;
        let mut longitude: Longitude = s[9..18].parse()?;

        let symbol_table = s.chars().nth(8).unwrap();
        let symbol_code = s.chars().nth(18).unwrap();

        let comment = &s[19..s.len()];

        // parse the comment
        let ogn = comment.parse::<PositionComment>().unwrap();

        // The comment may contain additional position precision information that will be added to the current position
        if let Some(precision) = &ogn.additional_precision {
            *latitude += precision.lat as f64 / 60_000.;
            *longitude += precision.lon as f64 / 60_000.;
        }

        // For fast distance calculations, we need to create a flat projection of the position
        let flat_projection = FlatProjection::new(*longitude, *latitude);
        let flat_point = flat_projection.project(*longitude, *latitude);

        Ok(AprsPosition {
            timestamp,
            messaging_supported,
            latitude,
            longitude,
            symbol_table,
            symbol_code,
            comment: ogn,
            flat_projection,
            flat_point,
        })
    }
}

impl AprsPosition {
    pub fn encode<W: Write>(&self, buf: &mut W) -> Result<(), EncodeError> {
        let sym = match (self.timestamp.is_some(), self.messaging_supported) {
            (true, true) => '@',
            (true, false) => '/',
            (false, true) => '=',
            (false, false) => '!',
        };

        write!(buf, "{}", sym)?;

        if let Some(ts) = &self.timestamp {
            write!(buf, "{}", ts)?;
        }

        write!(
            buf,
            "{}{}{}{}{:#?}",
            encode_latitude(self.latitude)?,
            self.symbol_table,
            encode_longitude(self.longitude)?,
            self.symbol_code,
            self.comment,
        )?;

        Ok(())
    }

    pub fn get_bearing(&self, other: &Self) -> f64 {
        self.flat_point.bearing(&other.flat_point)
    }

    pub fn get_distance(&self, other: &Self) -> f64 {
        self.flat_point.distance(&other.flat_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::WriterBuilder;
    use std::io::stdout;

    #[test]
    fn parse_without_timestamp_or_messaging() {
        let result = r"!4903.50N/07201.75W-".parse::<AprsPosition>().unwrap();
        assert_eq!(result.timestamp, None);
        assert_eq!(result.messaging_supported, false);
        assert_relative_eq!(*result.latitude, 49.05833333333333);
        assert_relative_eq!(*result.longitude, -72.02916666666667);
        assert_eq!(result.symbol_table, '/');
        assert_eq!(result.symbol_code, '-');
        assert_eq!(result.comment, PositionComment::default());
    }

    #[test]
    fn parse_with_comment() {
        let result = r"!4903.50N/07201.75W-Hello/A=001000"
            .parse::<AprsPosition>()
            .unwrap();
        assert_eq!(result.timestamp, None);
        assert_relative_eq!(*result.latitude, 49.05833333333333);
        assert_relative_eq!(*result.longitude, -72.02916666666667);
        assert_eq!(result.symbol_table, '/');
        assert_eq!(result.symbol_code, '-');
        assert_eq!(result.comment.unparsed.unwrap(), "Hello/A=001000");
    }

    #[test]
    fn parse_with_timestamp_without_messaging() {
        let result = r"/074849h4821.61N\01224.49E^322/103/A=003054"
            .parse::<AprsPosition>()
            .unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::HHMMSS(7, 48, 49)));
        assert_eq!(result.messaging_supported, false);
        assert_relative_eq!(*result.latitude, 48.36016666666667);
        assert_relative_eq!(*result.longitude, 12.408166666666666);
        assert_eq!(result.symbol_table, '\\');
        assert_eq!(result.symbol_code, '^');
        assert_eq!(result.comment.altitude.unwrap(), 003054);
        assert_eq!(result.comment.course.unwrap(), 322);
        assert_eq!(result.comment.speed.unwrap(), 103);
    }

    #[test]
    fn parse_without_timestamp_with_messaging() {
        let result = r"=4903.50N/07201.75W-".parse::<AprsPosition>().unwrap();
        assert_eq!(result.timestamp, None);
        assert_eq!(result.messaging_supported, true);
        assert_relative_eq!(*result.latitude, 49.05833333333333);
        assert_relative_eq!(*result.longitude, -72.02916666666667);
        assert_eq!(result.symbol_table, '/');
        assert_eq!(result.symbol_code, '-');
        assert_eq!(result.comment, PositionComment::default());
    }

    #[test]
    fn parse_with_timestamp_and_messaging() {
        let result = r"@074849h4821.61N\01224.49E^322/103/A=003054"
            .parse::<AprsPosition>()
            .unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::HHMMSS(7, 48, 49)));
        assert_eq!(result.messaging_supported, true);
        assert_relative_eq!(*result.latitude, 48.36016666666667);
        assert_relative_eq!(*result.longitude, 12.408166666666666);
        assert_eq!(result.symbol_table, '\\');
        assert_eq!(result.symbol_code, '^');
        assert_eq!(result.comment.altitude.unwrap(), 003054);
        assert_eq!(result.comment.course.unwrap(), 322);
        assert_eq!(result.comment.speed.unwrap(), 103);
    }

    #[ignore = "position_comment serialization not implemented"]
    #[test]
    fn test_serialize() {
        let aprs_position = r"@074849h4821.61N\01224.49E^322/103/A=003054"
            .parse::<AprsPosition>()
            .unwrap();
        let mut wtr = WriterBuilder::new().from_writer(stdout());
        wtr.serialize(aprs_position).unwrap();
        wtr.flush().unwrap();
    }

    #[test]
    fn test_input_string_too_short() {
        let result = "/13244".parse::<AprsPosition>();
        assert!(result.is_err(), "Short input string should return an error");
    }
}
