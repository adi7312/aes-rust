pub mod aes;
use aes::core::{AesCipher, AES};
use aes::modes::ctr::{CTR};

use pyo3::types::{PyBytes, PyType};
use pyo3::prelude::*;

#[pyclass(name = "CTR")]
struct PyCTR;

#[pymethods]
impl PyCTR {
    #[new]
    fn new() -> Self {
        PyCTR
    }
}

#[pyclass(name = "AES")]
struct PyAES {
    aes: Box<dyn AesCipher>,
}

#[pymethods]
impl PyAES {
    #[classmethod]
    fn init(_cls: Bound<'_, PyType>, mode: &Bound<'_, PyAny>, nonce: &[u8], key: &[u8]) -> PyResult<Self> {
        let key_array: [u8; 16] = key.try_into().expect("Key must be 16 bytes");
        let nonce_array: [u8; 8] = nonce.try_into().expect("Nonce must be 8 bytes");
        if mode.is_instance_of::<PyCTR>() {
            Ok(PyAES {
                aes: Box::new(AES::new(CTR, &key_array, Some(&nonce_array))),
            })
        } else {
            unimplemented!("Only CTR mode is implemented")
        }
    }

    fn encrypt<'p>(&self, py: Python<'p>, plaintext: &[u8]) -> PyResult<Py<PyBytes>> {
        let ciphertext = self.aes.encrypt(plaintext);
        Ok(PyBytes::new(py, &ciphertext).into())
    }

    fn decrypt<'p>(&self, py: Python<'p>, ciphertext: &[u8]) -> PyResult<Py<PyBytes>> {
        let plaintext = self.aes.decrypt(ciphertext);
        Ok(PyBytes::new(py, &plaintext).into())
    }
}


#[pymodule]
fn pyaes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAES>()?;
    m.add_class::<PyCTR>()?;
    Ok(())
}
