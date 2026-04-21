use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, io::Error};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: String,
    pub main_script: String,
    pub venv: Option<String>,
}

impl Project {
    pub fn new(
        name: String,
        version: String,
        description: String,
        main_script: String,
        venv: Option<String>,
    ) -> Project {
        Project {
            name,
            version,
            description,
            main_script,
            venv,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub project: Project,
    pub packages: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
}

impl Config {
    pub fn new(
        project: Project,
        packages: HashMap<String, String>,
        scripts: HashMap<String, String>,
    ) -> Config {
        Config {
            project,
            packages,
            scripts,
        }
    }

    pub fn write_to_file(&self, path: &str) -> Result<(), Error> {
        let toml_string =
            toml::to_string(&self).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(path, toml_string)
    }

    pub fn load_from_file(path: &str) -> Result<Config, Error> {
        let toml_string = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&toml_string)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    #[test]
    fn test_project_new() {
        let project = Project::new(
            "test_project".to_string(),
            "0.1.0".to_string(),
            "A test project".to_string(),
            "main.py".to_string(),
            Some("venv".to_string()),
        );

        assert_eq!(project.name, "test_project");
        assert_eq!(project.version, "0.1.0");
        assert_eq!(project.venv, Some("venv".to_string()));
    }

    #[test]
    fn test_config_save_load() {
        let project = Project::new(
            "test".to_string(),
            "1.0.0".to_string(),
            "desc".to_string(),
            "main.py".to_string(),
            None,
        );
        let mut packages = HashMap::new();
        packages.insert("requests".to_string(), "2.0.0".to_string());

        let config = Config::new(project, packages, HashMap::new());

        // Write to temp file
        let file = NamedTempFile::new().expect("Failed to create temp file");
        let path = file.path().to_str().unwrap();

        config.write_to_file(path).expect("Failed to write config");

        // Load back
        let loaded = Config::load_from_file(path).expect("Failed to load config");

        assert_eq!(loaded.project.name, "test");
        assert_eq!(loaded.packages.get("requests"), Some(&"2.0.0".to_string()));
    }
}
