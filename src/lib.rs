use pyo3::prelude::*;

#[pyfunction]
fn validate_and_process(data: String) -> PyResult<String> {
    if data.contains("attack") {
        // PySecurityError bulunamadığı için standart ValueError veya RuntimeError kullanıyoruz
        return Err(pyo3::exceptions::PyValueError::new_err("🛡️ RUST: Zararlı veri (attack) tespit edildi!"));
    }
    Ok(format!("Veri Rust tarafından onaylandı: {}", data))
}

#[pymodule]
fn rust_deserialization_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_and_process, m)?)?;
    Ok(())
}
