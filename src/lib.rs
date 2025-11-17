use pyo3::prelude::*;
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::OwnedBytes;

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


#[pymodule]
fn rsonpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    Ok(())
}
