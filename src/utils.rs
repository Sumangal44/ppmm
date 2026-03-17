use std::{
    io::{self, Write, IsTerminal},
    path::Path,
    process::Command,
};


// Constants
const PROJECT_CONFIG_FILE: &str = "project.toml";
const REQUIREMENTS_FILE: &str = "requirements.txt";
const PYPI_API_URL: &str = "https://pypi.org/pypi";

// Cross-platform path helpers
#[cfg(target_os = "windows")]
const PYTHON_EXE: &str = "python.exe";
#[cfg(not(target_os = "windows"))]
const PYTHON_EXE: &str = "python";

#[cfg(target_os = "windows")]
const PIP_EXE: &str = "pip.exe";
#[cfg(not(target_os = "windows"))]
const PIP_EXE: &str = "pip";

#[cfg(target_os = "windows")]
const VENV_BIN_DIR: &str = "Scripts";
#[cfg(not(target_os = "windows"))]
const VENV_BIN_DIR: &str = "bin";

pub fn get_venv_python_path(venv_root: &str) -> String {
    format!("./{}/{}/{}", venv_root, VENV_BIN_DIR, PYTHON_EXE)
}

pub fn get_venv_pip_path(venv_root: &str) -> String {
    format!("./{}/{}/{}", venv_root, VENV_BIN_DIR, PIP_EXE)
}

pub fn get_venv_bin_dir(venv_root: &str) -> String {
    format!("./{}/{}/", venv_root, VENV_BIN_DIR)
}

pub fn get_project_config_file() -> &'static str {
    PROJECT_CONFIG_FILE
}

pub fn get_requirements_file() -> &'static str {
    REQUIREMENTS_FILE
}

pub fn eprint(msg: String) {
    println!("{} {}", "error:".bright_red().bold(), msg.bright_red());
}

pub fn wprint(msg: String) {
    println!(
        "{} {}",
        "warning:".bright_yellow().bold(),
        msg.bright_yellow()
    );
}

pub fn iprint(msg: String) {
    println!(
        "{} {}",
        "•".bright_green().bold(),
        msg.bright_green().bold()
    );
}

pub fn project_exists(name: &String, is_init: bool) -> bool {
    if is_init {
        Path::new(get_project_config_file()).exists()
    } else {
        Path::new(name).exists()
        && Path::new(&format!("{}/{}", name, get_project_config_file())).exists()
    }
}

pub fn check_venv_dir_exists(venv_root: &str) -> bool {
    Path::new(&get_venv_bin_dir(venv_root)).exists()
}

pub fn get_pkg_version(pkg: &str) -> Result<String, String> {
    let url = format!("{}/{}/json", PYPI_API_URL, pkg);
    let resp = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to retrieve package version: {}", e))?;

    let json: serde_json::Value = resp
        .json()
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    let version = json["info"]["version"]
        .as_str()
        .ok_or_else(|| "Version field not found in response".to_string())?;

    Ok(version.to_string())
}

pub fn setup_venv(venv_path: String) -> Result<(), String> {
    iprint("Setting Up Virtual Environment...".to_string());
    let venv = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(&venv_path)
        .output()
        .map_err(|e| format!("Failed to execute python command: {}", e))?;

    if !venv.status.success() {
        return Err(format!(
            "Virtual environment creation failed: {}",
            String::from_utf8_lossy(&venv.stderr)
        ));
    }
    Ok(())
}

pub fn ask_if_create_venv() -> bool {
    // Check if stdin is a terminal
    if !std::io::stdin().is_terminal() {
        wprint("Non-interactive environment detected. Defaulting to 'yes' for venv creation.".to_string());
        return true;
    }

    loop {
        let mut answer = String::new();
        print!(
            "{}",
            "[?] Do you want to create a virtual environment? (y/n): "
                .green()
                .bold()
        );
        
        // Handle potential flush error safely
        if let Err(e) = io::stdout().flush() {
            eprint(format!("Failed to flush stdout: {}", e));
            return false; // Default safe behavior
        }

        // Handle read_line error safely
        if let Err(e) = io::stdin().read_line(&mut answer) {
            eprint(format!("Failed to read line: {}", e));
            return false;
        }

        match answer.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("{}", "Invalid option, please type 'y' or 'n'".bright_red());
            }
        }
    }
}

pub fn parse_version(pkg: &str) -> (String, Option<String>) {
    if let Some((name, version)) = pkg.split_once("==") {
        (name.to_string(), Some(version.to_string()))
    } else {
        (pkg.to_string(), None)
    }
}

fn validate_package_name(pkg: &str) -> Result<(), String> {
    if pkg.is_empty() {
        return Err("Package name cannot be empty".to_string());
    }
    if pkg
        .chars()
        .any(|c| !c.is_alphanumeric() && !"._-=<>~!".contains(c))
    {
        return Err(format!("Invalid package name: {}", pkg));
    }
    Ok(())
}


pub fn install_packages_batch(pkgs: &[String], venv_root: &str) -> Result<(), String> {
    if !check_venv_dir_exists(venv_root) {
        return Err("Virtual Environment Not Found".to_string());
    }

    if pkgs.is_empty() {
        return Ok(());
    }

    for pkg in pkgs {
        validate_package_name(pkg)?;
    }

    iprint(format!("Installing {} packages...", pkgs.len()));
    let output = Command::new(get_venv_pip_path(venv_root))
        .arg("install")
        .args(pkgs)
        .output()
        .map_err(|e| format!("Failed to execute pip: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to install packages: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

pub fn generate_lock_file(venv_root: &str) -> Result<(), String> {
    if !check_venv_dir_exists(venv_root) {
        return Err("Virtual Environment Not Found".to_string());
    }

    iprint("Generating ppmm.lock...".to_string());
    let output = Command::new(get_venv_pip_path(venv_root))
        .arg("freeze")
        .output()
        .map_err(|e| format!("Failed to execute pip freeze: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to generate lock file: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let lock_content = String::from_utf8_lossy(&output.stdout);
    let mut file = std::fs::File::create("ppmm.lock")
        .map_err(|e| format!("Failed to create ppmm.lock: {}", e))?;
    
    file.write_all(lock_content.as_bytes())
        .map_err(|e| format!("Failed to write to ppmm.lock: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name() {
        assert!(validate_package_name("requests").is_ok());
        assert!(validate_package_name("my-package").is_ok());
        assert!(validate_package_name("my_package").is_ok());
        assert!(validate_package_name("package123").is_ok());
        
        // Invalid names
        assert!(validate_package_name("").is_err());
        assert!(validate_package_name("pkg with spaces").is_err());
        assert!(validate_package_name("pkg/slash").is_err());
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("requests==2.26.0"), ("requests".to_string(), Some("2.26.0".to_string())));
        assert_eq!(parse_version("numpy"), ("numpy".to_string(), None));
    }

    #[test]
    fn test_get_venv_paths() {
        let venv_root = "test_venv";
        
        #[cfg(target_os = "windows")]
        {
            assert_eq!(get_venv_python_path(venv_root), "./test_venv/Scripts/python.exe");
            assert_eq!(get_venv_pip_path(venv_root), "./test_venv/Scripts/pip.exe");
            assert_eq!(get_venv_bin_dir(venv_root), "./test_venv/Scripts/");
        }

        #[cfg(not(target_os = "windows"))]
        {
            assert_eq!(get_venv_python_path(venv_root), "./test_venv/bin/python");
            assert_eq!(get_venv_pip_path(venv_root), "./test_venv/bin/pip");
            assert_eq!(get_venv_bin_dir(venv_root), "./test_venv/bin/");
        }
    }
}
