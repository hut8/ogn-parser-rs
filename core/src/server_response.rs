use std::str::FromStr;

use serde::Serialize;

use crate::AprsError;
use crate::AprsPacket;
use crate::ServerComment;

#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerResponse {
    AprsPacket(AprsPacket),
    ServerComment(ServerComment),
    ParserError(AprsError),
}

impl FromStr for ServerResponse {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<ServerResponse, Self::Err> {
        if s.starts_with("#") {
            match ServerComment::from_str(s) {
                Ok(server_comment) => Ok(ServerResponse::ServerComment(server_comment)),
                Err(err) => Ok(ServerResponse::ParserError(err)),
            }
        } else {
            match AprsPacket::from_str(s) {
                Ok(aprs_packet) => Ok(ServerResponse::AprsPacket(aprs_packet)),
                Err(err) => Ok(ServerResponse::ParserError(err)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_aprs_packet() {
        let result = r"ICA3D17F2>APRS,qAS,dl4mea:/074849h4821.61N\01224.49E^322/103/A=003054 !W46! id213D17F2 -039fpm +0.0rot 2.5dB 3e -0.0kHz gps1x1".parse::<ServerResponse>();
        if let Ok(ServerResponse::AprsPacket(_)) = result {
            // Do nothing, we expect this to be an AprsPacket
        } else {
            panic!("Expected AprsPacket, got something else");
        }
    }

    #[test]
    fn parse_server_comment() {
        let result =
            r"# aprsc 2.1.4-g408ed49 17 Mar 2018 09:30:36 GMT GLIDERN1 37.187.40.234:10152"
                .parse::<ServerResponse>();
        if let Ok(ServerResponse::ServerComment(_)) = result {
            // Do nothing, we expect this to be a ServerComment
        } else {
            panic!("Expected ServerComment, got something else");
        }
    }

    #[test]
    fn parse_invalid() {
        let result = r"INVALID APRS PACKET".parse::<ServerResponse>();
        if let Ok(ServerResponse::ParserError(_)) = result {
            // Do nothing, we expect this to be an InvalidWtf
        } else {
            panic!("Expected InvalidWtf, got something else");
        }
    }
}
