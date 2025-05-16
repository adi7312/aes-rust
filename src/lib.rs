pub mod aes;
use aes::core::{AES};
use aes::modes::ctr::CTR;

use pyo3::types::PyBytes;
use pyo3::prelude::*;


#[pyfunction]
fn encrypt(py: Python, key: &[u8], plaintext: &[u8])-> PyResult<Py<PyBytes>>{
    let key_array: [u8; 16] = key.try_into().expect("Key length must be a multiple 16 bytes");
    let aes = AES::new(CTR, &key_array);
    let ciphertext = aes.encrypt(plaintext);
    Ok(PyBytes::new(py, &ciphertext).into())
}

#[pyfunction]
fn decrypt(py: Python, key: &[u8], ciphertext: &[u8])-> PyResult<Py<PyBytes>>{
    let key_array: [u8; 16] = key.try_into().expect("Key length must be a multiple 16 bytes");
    let aes = AES::new(CTR, &key_array);
    let plaintext = aes.decrypt(ciphertext);
    Ok(PyBytes::new(py, &plaintext).into())
}

#[pymodule]
fn pyaes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decrypt, m)?)?;
    m.add_function(wrap_pyfunction!(encrypt, m)?)?;
    Ok(())
}
