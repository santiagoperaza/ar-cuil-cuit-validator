#[cfg(test)]
use super::*;

#[test]
fn assert_validation_returns_true() {
    let valid_cuil_cuit = 27280335148;
    let result = is_valid(valid_cuil_cuit).unwrap();
    assert_eq!(true, result);
}

#[test]
fn assert_validation_returns_false() {
    let invalid_cuil_cuit = 27280335141;
    let result = is_valid(invalid_cuil_cuit).unwrap();
    assert_eq!(false, result);
}

#[test]
fn assert_validation_fails_due_to_invalid_cuil_cuit_format() {
    let invalid_cuil_cuit_format = 27280;
    let result = is_valid(invalid_cuil_cuit_format);
    assert!(result
        .unwrap_err()
        .to_string()
        .starts_with("Invalid CUIL/CUIT format, it must be 11 digits"));
}
