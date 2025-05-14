use ogn_parser::ServerResponse;
use pyo3::prelude::*;
use pythonize::pythonize;
use rayon::prelude::*;

/// Parse an APRS packet from a string to a list of JSON strings: List[str]
#[pyfunction]
fn parse_to_json(s: &str) -> PyResult<Vec<String>> {
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
fn parse(py: Python, s: &str) -> PyResult<Py<PyAny>> {
    let lines = s.lines().collect::<Vec<_>>();
    Ok(pythonize(
        py,
        &lines
            .par_iter()
            .map(|&aprs_string| aprs_string.parse::<ServerResponse>().unwrap())
            .collect::<Vec<_>>(),
    )?
    .into())
}

/// A Python module implemented in Rust.
#[pymodule(name = "ogn_parser")]
fn python_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
