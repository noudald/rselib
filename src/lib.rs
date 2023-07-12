use pyo3::prelude::*;


#[pyfunction]
fn rot13(message: String) -> PyResult<String> {
    Ok(message
        .chars()
        .map(|c| match c {
            'A' ... 'M' | 'a' ... 'm' => ((c as u8) + 13) as char,
            'N' ... 'Z' | 'n' ... 'z' => ((c as u8) - 13) as char,
            _ => c
        })
        .collect()
    )
}

#[pymodule]
fn rselib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rot13, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::rot13;

    #[test]
    fn test_rot13() {
        let message = "This is a message";
        assert!(message != rot13(message.to_string()).unwrap());
        assert_eq!(message, rot13(rot13(message.to_string()).unwrap()).unwrap());
    }
}
