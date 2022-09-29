use anyhow::{ Context, Result };
use serde::{ de::DeserializeOwned, Serialize };
use std::{ fs, path::PathBuf };

/// Read data from yaml as struct T
pub fn read_yaml<T: DeserializeOwned + Default>(path: PathBuf) -> T {
  let yaml_str = fs::read_to_string(path).unwrap_or("".into());
  serde_yaml::from_str::<T>(&yaml_str).unwrap_or(T::default())
}

/// Save the data to the file - can set `prefix` string to add some comments
pub fn save_yaml<T: Serialize>(path: PathBuf, data: &T, prefix: Option<&str>) -> Result<()> {
  let data_str = serde_yaml::to_string(data)?;

  let yaml_str = match prefix {
    Some(prefix) => format!("{prefix}{data_str}"),
    None => data_str,
  };

  let path_str = path.as_os_str().to_string_lossy().to_string();
  fs::write(path, yaml_str.as_bytes()).context(format!("failed to save file \"{path_str}\""))
}