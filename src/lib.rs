mod query_string;

use pyo3::prelude::*;
use pythonize::pythonize;

pub use query_string::{parse_query_string as parse_qs, parse_query_string_to_json};

// parse query string into a list of tuples.
#[pyfunction]
#[pyo3(text_signature = "(qs, separator)")]
fn parse_query_string(qs: &[u8], separator: char) -> PyResult<Vec<(String, String)>> {
    Ok(parse_qs(qs, separator))
}

// parse query string into a python object.
#[pyfunction]
#[pyo3(signature = (qs, parse_numbers=true),text_signature = "(qs, parse_numbers=true)")]
fn parse_url_encoded_dict(py: Python, qs: &[u8], parse_numbers: bool) -> PyResult<Py<PyAny>> {
    Ok(
        pythonize(py, &parse_query_string_to_json(qs, parse_numbers))
            .unwrap()
            .into(),
    )
}

#[pymodule]
mod fast_query_parsers {
    use super::*;
    #[pyfunction]
    #[pyo3(name = "parse_query_string")]
    fn parse_query_string_py(
        py: Python,
        qs: &[u8],
        separator: char,
    ) -> PyResult<Py<PyAny>> {
        let result = parse_query_string(qs, separator)?;
        Ok(pythonize(py, &result).unwrap().into())
    }

    #[pyfunction]
    #[pyo3(name = "parse_url_encoded_dict",signature = (qs, parse_numbers=true),text_signature = "(qs, parse_numbers=true)")]
    fn parse_url_encoded_dict_py(
        py: Python,
        qs: &[u8],
        parse_numbers: bool,
    ) -> PyResult<Py<PyAny>> {
        parse_url_encoded_dict(py, qs, parse_numbers)
    }
}
