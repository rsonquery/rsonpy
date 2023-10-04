use pyo3::prelude::*;
use pyo3::types::{PyString, PyStringData};

use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::{OwnedBytes, MmapInput};
use rsonpath::query::JsonPathQuery;
use std::fs::File;

#[pyclass]
struct PyQuery{
    query: JsonPathQuery,
}

impl PyQuery{
    pub fn compile(&self) -> RsonpathEngine{
        RsonpathEngine::compile_query(&self.query).unwrap()
    }
}

#[pymethods]
impl PyQuery {
    #[new]
    fn new(query: &str) -> Self {
        PyQuery { query: JsonPathQuery::parse(query).unwrap() }
    }

    unsafe fn loads(&self, s: &PyString) -> PyResult<Vec<(usize, usize)>>{
        let contents = s.data()?;
        let raw_data =  match contents {
            PyStringData::Ucs1(raw_data) => raw_data,
            _ => panic!("Not UTF-8 encoded string")
        };
        let input = OwnedBytes::new(&raw_data).unwrap();
        let mut results = vec![];
        let _ = &self.compile().approximate_spans(&input, &mut results).unwrap();
        let cast = results.into_iter().map(|m| (m.start_idx(), m.end_idx()) ).collect(); 
        Ok(cast)
    }

    unsafe fn load(&self, path: &str) -> PyResult<Vec<String>> {
        let file = File::open(path).unwrap();
        let input = MmapInput::map_file(&file).unwrap(); 
        let mut results = vec![];
        let _ = &self.compile().matches(&input, &mut results).unwrap();
        let cast = results.into_iter().map(|m| String::from_utf8(m.into_bytes()).unwrap() ).collect(); 
        Ok(cast)
    }
    
    unsafe fn count_f(&self, path: &str) -> PyResult<u64> {
        let file = File::open(path).unwrap();
        let input = MmapInput::map_file(&file).unwrap(); 
        Ok(self.compile().count(&input).unwrap())
    }

    unsafe fn count_s(&self, s: &PyString) -> PyResult<u64> {
        let contents = s.data()?;
        let raw_data =  match contents {
            PyStringData::Ucs1(raw_data) => raw_data,
            _ => panic!("Not UTF-8 encoded string")
        };
        let input = OwnedBytes::new(&raw_data).unwrap();
        Ok(self.compile().count(&input).unwrap())
    }

}


/// A Python module implemented in Rust.
#[pymodule]
fn rsonpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyQuery>()?;
    Ok(())
}
