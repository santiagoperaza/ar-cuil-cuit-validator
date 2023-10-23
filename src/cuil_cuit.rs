use std::fmt;

#[derive(Debug)]
pub enum CuilCuitError {
    InvalidFormat(String),
}

impl fmt::Display for CuilCuitError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let message = match self {
        CuilCuitError::InvalidFormat(cuil_cuit) => format!("Invalid CUIL/CUIT format, it must be 11 digits, received: {}", cuil_cuit),
      };
      write!(f, "{}", message)
  }
}

pub fn is_valid(cuil_cuit: u64) -> Result<bool, CuilCuitError> {
  if cuil_cuit.to_string().len() != 11 {
    Err(CuilCuitError::InvalidFormat(cuil_cuit.to_string()))
  } else {
    let expected_last_digit = calculate_last_digit(cuil_cuit);
    let last_digit = cuil_cuit.to_string().chars().last().unwrap().to_digit(10).unwrap();
    if last_digit == expected_last_digit {
        Ok(true)
    } else {
        Ok(false)
    }
  }
}

fn calculate_last_digit(cuil_cuit: u64) -> u32 {
  let digits: Vec<_> = cuil_cuit.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
  let digits = &digits[..10];

  let validators = vec![5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
  let mut sum = 0;
  for (index, ele) in digits.iter().enumerate() {
      sum += validators[index] * ele
  }

  let mod11 = 11 - (sum % 11);

  if mod11 == 11 { 0 } else { mod11 }
}

#[cfg(test)]
mod test;
