
#![cfg_attr(target_os = "windows", windows_subsystem = "console")]
// ^^ "windows" will not allocate a console on windows

use eframe::egui;

use pyo3::prelude::*;

static PYEMBEDDED_STDLIB_DIR: include_dir::Dir<'_> = include_dir::include_dir!("pyembedded/stdlib");

fn main() {
  // On windows, we compile as a console app and dynamically hide the console;
  // if launched from start menu / explorer.exe, no console.
  // If launched from console, keep console.
  #[cfg(target_os = "windows")]
  hide_console_on_windows_win();

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

  // GUI experiment
  {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    );
  }


  let r = {
    pyo3::prepare_freethreaded_python();
    pyo3::Python::with_gil(|py| {
        py.run("print('hello, world'); import code; code.interact()", None, None)
    })
  };
  if let Err(e) = r {
    if format!("{:?}", e).contains("value: SystemExit") {
      // NOP
    }
    else {
      println!("{:?}", e);
    }
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


#[cfg(target_os = "windows")]
fn hide_console_on_windows_win() {
    // Check if we are run from the console or just launched with explorer.exe
    let mut console_proc_list_buff: Vec<u32> = vec![0; 16];
    let num_procs = unsafe {
        winapi::um::wincon::GetConsoleProcessList(console_proc_list_buff.as_mut_ptr(), 16)
    };
    //eprintln!("num_procs={:?}", num_procs);
    if num_procs == 1 || num_procs == 2 {
        // We were launched from explorer.exe, detatch the console
        unsafe { winapi::um::wincon::FreeConsole() };
    }
    // Otherwise do nothing, we want console messages when run from the console.
}




struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            /*ui.image(egui::include_image!(
                "../../../crates/egui/assets/ferris.png"
            ));*/

        });
    }
}


