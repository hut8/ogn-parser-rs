extern crate ogn_parser;

fn main() {
    let result = ogn_parser::parse(
        r"ICA3D1C35>OGFLR,qAS,Padova:/094220h4552.41N/01202.28E'110/099/A=003982 !W96! id053D1C35 -1187fpm +0.0rot 0.8dB 2e +4.5kHz gps1x2 s6.09 h32 rDD09D0",
    );

    println!("{:#?}", result);
}
