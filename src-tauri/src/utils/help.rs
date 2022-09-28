#[macro_export]
macro_rules! log_if_err {
  ($result:expr) => {
    if let Err(err) = $result {
      log::error!("{err}");
    }
  };
}

/// Wrap the anyhow error, transforming the error to String
#[macro_export]
macro_rules! wrap_err {
  ($stat:expr) => {
    match $stat {
      Ok(a) => Ok(a),
      Err(err) => {
        log::error!("{}", err.to_string());
        Err(format!("{}", err.to_string()))
      }
    }
  };
}

/// Return the string literal error
#[macro_export]
macro_rules! ret_err {
  ($str:expr) => {
    return Err($str.into())
  };
}