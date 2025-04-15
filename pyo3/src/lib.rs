use ogn_parser::AprsPacket;
use pyo3::prelude::*;
use pythonize::pythonize;
use rayon::prelude::*;

/// Parse an APRS packet from a string to a list of JSON strings: List[str]
#[pyfunction]
fn parse_raw(packet: &str) -> PyResult<Vec<String>> {
    let aprs_strings = packet.lines().collect::<Vec<_>>();
    let json_strings = aprs_strings
        .par_iter()
        .map(|aprs_string| {
            serde_json::to_string(&aprs_string.parse::<AprsPacket>().unwrap()).unwrap()
        })
        .collect();
    Ok(json_strings)
}

/// Parse an APRS packet from a string to a Python object: List[Dict[str, Any]]
#[pyfunction]
fn parse(py: Python, packet: &str) -> PyResult<Py<PyAny>> {
    let aprs_strings = packet.lines().collect::<Vec<_>>();
    let packets = aprs_strings
        .par_iter()
        .map(|aprs_string| aprs_string.parse::<AprsPacket>().unwrap())
        .collect::<Vec<AprsPacket>>();
    Ok(pythonize(py, &packets)?.into())
}

/// A Python module implemented in Rust.
#[pymodule(name = "ogn_parser")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_raw, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
