use ogn_parser::{AprsData, AprsPacket, ServerResponse};
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
};
use pythonize::pythonize;
use rayon::prelude::*;

/// Parse an APRS packet from a string to a list of JSON strings: List[str]
#[pyfunction]
fn parse_serde_json(s: &str) -> PyResult<Vec<String>> {
    let lines = s.lines().collect::<Vec<_>>();
    let json_strings = lines
        .par_iter()
        .map(|&aprs_string| {
            serde_json::to_string(&aprs_string.parse::<ServerResponse>().unwrap()).unwrap()
        })
        .collect();
    Ok(json_strings)
}

/// Parse an APRS packet from a string to a Python object: List[Dict[str, Any]]
#[pyfunction]
fn parse_pythonize(py: Python, s: &str) -> PyResult<Py<PyAny>> {
    let lines = s.lines().collect::<Vec<_>>();
    let packets = lines
        .par_iter()
        .map(|&aprs_string| aprs_string.parse::<ServerResponse>().unwrap())
        .collect::<Vec<_>>();
    Ok(pythonize(py, &packets)?.into())
}

// Parse an APRS packet from a string to a Python dict: Dict[str, Any]
#[pyfunction]
fn parse_pyo3(py: Python, s: &str) -> PyResult<Py<PyList>> {
    let lines = s.lines().collect::<Vec<_>>();
    let packets = lines
        .par_iter()
        .map(|aprs_string| aprs_string.parse::<AprsPacket>().unwrap())
        .collect::<Vec<AprsPacket>>();
    let dicts = PyList::empty(py);
    for packet in packets {
        let dict = PyDict::new(py);
        dict.set_item("raw_string", s).unwrap();
        match packet.data {
            AprsData::Position(ref pos) => {
                dict.set_item("latitude", *pos.latitude).unwrap();
                dict.set_item("longitude", *pos.longitude).unwrap();
            }
            AprsData::Status(ref _status) => {}
            _ => {}
        };
        dicts.append(dict).unwrap();
    }

    Ok(dicts.into())
}

/// A Python module implemented in Rust.
#[pymodule(name = "ogn_parser")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_serde_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_pythonize, m)?)?;
    m.add_function(wrap_pyfunction!(parse_pyo3, m)?)?;
    Ok(())
}
