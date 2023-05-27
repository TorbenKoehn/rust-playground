use std::path::PathBuf;

const POSSIBLE_DATA_PATHS: &'static [&str] =
  &["{{ user_dir }}/Documents/Larian Studios/Divinity Original Sin Enhanced Edition"];

pub fn get_dos_ee_data_path() -> Option<PathBuf> {
  let user_dirs: Vec<PathBuf> = vec![dirs_next::home_dir()?.join("Documents")];
  // Create a matrix replacing all {{ user_dir }} instances in all POSSIBLE_DATA_PATHS with all user_dirs
  let possible_data_paths: Vec<String> = POSSIBLE_DATA_PATHS
    .iter()
    .flat_map(|possible_data_path| {
      user_dirs
        .iter()
        .map(|user_dir| {
          let possible_data_path =
            possible_data_path.replace("{{ user_dir }}", &user_dir.to_string_lossy());
          possible_data_path
        })
        .collect::<Vec<_>>()
    })
    .collect();
  // Check if any of the possible data paths exist
  for possible_data_path in possible_data_paths.iter() {
    if PathBuf::from(possible_data_path).exists() {
      return Some(PathBuf::from(possible_data_path));
    }
  }
  None
}
