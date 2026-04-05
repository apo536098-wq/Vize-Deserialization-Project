use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// KRİTİK SATIR: Tanımlanmamış her alanı (attack vb.) anında reddeder!
#[serde(deny_unknown_fields)] 
struct SecurePayload {
    username: String,
    user_id: i32,
    role: String,
}

#[pyfunction]
fn validate_and_process(json_data: String) -> PyResult<String> {
    let result: Result<SecurePayload, serde_json::Error> = serde_json::from_str(&json_data);
    
    match result {
        Ok(data) => Ok(format!(
            "Giriş Başarılı: {}, ID: {}, Rol: {}", 
            data.username, data.user_id, data.role
        )),
        Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "GÜVENLİK İHLALİ: Geçersiz, eksik veya zararlı veri yapısı tespit edildi!"
        )),
    }
}

#[pymodule]
fn rust_security_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_and_process, m)?)?;
    Ok(())
}
