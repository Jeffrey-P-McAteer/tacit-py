use pyo3::prelude::*;

static PYEMBEDDED_STDLIB_DIR: include_dir::Dir<'_> = include_dir::include_dir!("pyembedded/stdlib");

fn main() -> PyResult<()> {
  // We embed the folder pyembedded/stdlib at build-time; at run-time python expects to find
  // this at PYTHONPATH, so we extract & assign PYTHONPATH to the system temp dir.
  let mut pyembedded_stdlib = std::env::temp_dir();
  pyembedded_stdlib.push("pyembedded_stdlib");
  let pyembedded_stdlib = pyembedded_stdlib;
  if let Err(e) = std::fs::create_dir_all(&pyembedded_stdlib) {
    println!("{:?}", e);
  }
  extract_children(&PYEMBEDDED_STDLIB_DIR, &pyembedded_stdlib);
  std::env::set_var("PYTHONPATH", pyembedded_stdlib.into_os_string() );
  unsafe {
    pyo3::with_embedded_python_interpreter(|py| {
        py.run("print('hello, world'); import code; code.interact()", None, None)
    })
  }
}

fn extract_children(embedded_dir: &include_dir::Dir, real_dir: &std::path::PathBuf) {
  for entry in embedded_dir.entries() {
    match entry {
      include_dir::DirEntry::Dir(entry_embedded_dir) => {
        let mut entry_real_dir = real_dir.clone();
        if let Some(entry_path_file_name) = entry.path().file_name() {
          entry_real_dir.push(entry_path_file_name);
          let entry_real_dir = entry_real_dir;
          if let Err(e) = std::fs::create_dir_all(&entry_real_dir) {
            println!("{:?}", e);
          }
          extract_children(&entry_embedded_dir, &entry_real_dir);
        }
      }
      include_dir::DirEntry::File(entry_embedded_file) => {
        let mut entry_real_file = real_dir.clone();
        if let Some(entry_path_file_name) = entry.path().file_name() {
          entry_real_file.push(entry_path_file_name);
          let entry_real_file = entry_real_file;
          if !entry_real_file.exists() {
            // extract
            if let Err(e) = std::fs::write(&entry_real_file, entry_embedded_file.contents() ) {
              println!("{:?}", e);
            }
          }
        }
      }
    }
  }
}

