// include!("../pyembedded/default_python_config.rs");

fn main() {
    // Get config from default_python_config.rs.
    //let config = default_python_config();

    let mut config = pyembed::OxidizedPythonInterpreterConfig::default();

    // See https://github.com/indygreg/PyOxidizer/issues/702#issuecomment-1774217494
    config.interpreter_config.isolated = Some(true);
    config.interpreter_config.filesystem_encoding = Some("utf-8".to_string());
    config.set_missing_path_configuration = false;
    config.interpreter_config.parse_argv = Some(false);
    config.argv = Some(vec!["python".into()]);
    config.interpreter_config.executable = Some("python".into());

    let interp = pyembed::MainPythonInterpreter::new(config).unwrap();

    // `py` is a `pyo3::Python` instance.
    interp.with_gil(|py| {
        py.run("import sys; print('hello, world! I am python ', sys.version, '!')", None, None).unwrap();
    });

}
