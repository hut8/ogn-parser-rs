use aprs_parser::AprsPacket;
use pyo3::prelude::*;
use rayon::prelude::*;

/// Parse an APRS packet from a string.
#[pyfunction]
fn parse(packet: &str) -> PyResult<Vec<String>> {
    let aprs_strings = packet.lines().collect::<Vec<_>>();
    let dicts = aprs_strings
        .par_iter()
        .map(|aprs_string| {
            serde_json::to_string(&aprs_string.parse::<AprsPacket>().unwrap()).unwrap()
        })
        .collect();
    Ok(dicts)
}

/// A Python module implemented in Rust.
#[pymodule(name = "aprs_parser")]
fn aprs_parser_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
