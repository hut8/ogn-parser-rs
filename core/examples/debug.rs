extern crate ogn_parser;

fn main() {
    let result = ogn_parser::parse(
        r"Herborn>OGNSDR,TCPIP*,qAC,GLIDERN5:>225942h v0.3.2.arm64 CPU:0.7 RAM:790.3/3976.3MB NTP:0.3ms/-15.3ppm +61.3C EGM96:+49m 0/0Acfts[1h] RF:+0+0.0ppm/+4.10dB/+2.9dB@10km[751698]/+10.1dB@10km[1/2]",
    );

    println!("{:#?}", result);
}
