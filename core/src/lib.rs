use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashSet;

#[pyfunction]
fn scan_contract(py: Python, code: &str) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    
    let patterns = vec![
        ("critical_reentrancy_risk", r"\.call\{value:.*\}\([^)]*\)"),
        ("reentrancy_risk", r"\.call\.value\("),
        ("unprotected_delegatecall", r"delegatecall"),
        ("selfdestruct_risk", r"selfdestruct"),
        ("tx_origin_authentication_risk", r"tx\.origin"),
        ("unchecked_external_call", r"\.call\("),
        ("unchecked_send", r"\.send\("),
        ("uninitialized_proxy_pattern", r"upgradeTo"),
        ("spot_price_oracle_manipulation", r"getReserves"),
        ("flash_loan_attack_vector", r"flashloan|executeOperation"),
        ("signature_malleability_risk", r"ecrecover"),
        ("governance_delay_risk", r"propose|vote"),
        ("arbitrary_jump_dest", r"assembly.*jump"),
    ];
    
    let findings: HashSet<&str> = patterns.par_iter()
        .filter(|(_, p)| {
            code.contains(p.replace("\\", "").as_str()) 
        })
        .map(|(name, _)| *name)
        .collect();
    
    let findings_vec: Vec<&str> = findings.into_iter().collect();
    
    dict.set_item("vulnerabilities", &findings_vec)?;
    dict.set_item("has_threats", !findings_vec.is_empty())?;
    dict.set_item("scan_engine", "Aura-Rust-Elite")?;
    
    Ok(dict.into())
}

#[pymodule]
fn aura_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scan_contract, m)?)?;
    Ok(())
}
