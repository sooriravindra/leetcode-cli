//! This module is for python scripts.
//!
//! Seems like some error exists now, welocome pr to fix this : )
use crate::cache::Cache;
use crate::helper::load_script;
use pyo3::prelude::*;

/// Exec python scripts as filter
pub fn exec(module: &str) -> Result<Vec<String>, crate::Error> {
    let script = load_script(&module)?;
    let cache = Cache::new()?;

    pyo3::prepare_freethreaded_python();
    // pygil
    Python::with_gil(|py| {
        let fun_plan = PyModule::from_code(py, &script,
            "plan_script.py","plan_script")?.getattr("plan")?;

    // args
    let sps = serde_json::to_string(&cache.get_problems()?)?;
    let stags = serde_json::to_string(&cache.get_tags()?)?;
    let args = (sps, stags);

    // ret
    let res: Vec<String> = fun_plan.call1(args)?.extract()?;
    // let res: Vec<String> = pym.call1((sps, stags))?.extract()?;

    Ok(res)
    })
}
