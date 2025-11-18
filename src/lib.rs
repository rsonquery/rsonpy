use pyo3::prelude::*;
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::{OwnedBytes, MmapInput};
use std::fs::File;

#[pyfunction]
fn loads<'a>(s: &'a str, query_str: &'a str) -> PyResult<Vec<(usize, usize)>>{
    let query = rsonpath_syntax::parse(query_str).unwrap();
    let contents = s.to_owned();
    let input = OwnedBytes::try_from(contents).unwrap();
    let engine = RsonpathEngine::compile_query(&query).unwrap();
    let mut results = vec![];
    engine.approximate_spans(&input, &mut results).unwrap();
    let cast = results.into_iter().map(|m| (m.start_idx(), m.end_idx()) ).collect(); 
    Ok(cast)
}

#[pyfunction]
fn load<'a>(input_file: &'a str, query_str: &'a str) -> PyResult<Vec<Vec<u8>>>{
    let query = rsonpath_syntax::parse(query_str).unwrap();
    let file = File::open(input_file).unwrap();
    // SAFETY: File is kept open until end of the run.
    let input = unsafe { MmapInput::map_file(&file).unwrap() };
    let engine = RsonpathEngine::compile_query(&query).unwrap();
    let mut results = vec![];
    engine.matches(&input, &mut results).unwrap();
    let cast = results.into_iter().map(|m| m.into_bytes()).collect(); 
    Ok(cast)
}


#[pymodule]
fn rsonpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
