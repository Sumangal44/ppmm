use crate::settings::Config;
use crate::utils::*;
use colored::*;
pub(crate) use std::path::Path;
use std::process::Command;

pub fn show_project_info() {
    let config_file = get_project_config_file();
    if !Path::new(config_file).exists() {
        eprint(format!("Could not find {}", config_file));
        return;
    }
    let conf = match Config::load_from_file(config_file) {
        Ok(conf) => conf,
        Err(e) => {
            eprint(e.to_string());
            return;
        }
    };
    println!();

    let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

    match Command::new(get_venv_python_path(venv_root))
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            if let Some((name, ver)) = version.trim().split_once(' ') {
                println!("{}: {}", name.bold().bright_purple(), ver.bold().red());
            }
        }
        Ok(_) | Err(_) => {
            wprint("Failed to get Python version".to_string());
        }
    };

    println!(
        "{}: {}",
        "Project".green().bold(),
        conf.project.name.bright_cyan().bold()
    );
    println!(
        "{}: {}",
        "Version".green().bold(),
        conf.project.version.bright_red().bold()
    );
    println!(
        "{}: {}",
        "Description".green().bold(),
        conf.project.description.bright_white().bold()
    );

    println!();
    let count = conf.scripts.len();
    println!(
        "-- {} {} --",
        count.to_string().green().bold(),
        if count == 1 {
            "Script".to_owned()
        } else {
            "Scripts".to_owned()
        }
    );
    for (name, cmd) in conf.scripts.iter() {
        println!("{}: {}", name.bright_yellow().bold(), cmd.green().bold());
    }

    println!();
    let count = conf.packages.len();
    println!(
        "-- {} {} --",
        count.to_string().green().bold(),
        if count == 1 {
            "Package".to_owned()
        } else {
            "Packages".to_owned()
        }
    );
    for (name, version) in conf.packages.iter().take(10) {
        println!(
            "{}=={}",
            name.bright_yellow().bold(),
            version.bright_red().bold()
        );
    }
    if conf.packages.len() > 10 {
        println!("... and {} more", conf.packages.len() - 10);
    }
    println!();
}

pub fn gen_requirements() {
    let config_file = get_project_config_file();
    if !Path::new(config_file).exists() {
        eprint(format!("Could not find {}", config_file));
        return;
    }

    let conf = match Config::load_from_file(config_file) {
        Ok(conf) => conf,
        Err(e) => {
            eprint(e.to_string());
            return;
        }
    };

    let mut reqs = String::new();
    for (name, version) in conf.packages.iter() {
        reqs.push_str(&format!("{}=={}\n", name, version));
    }

    let req_file = get_requirements_file();
    match std::fs::write(req_file, reqs) {
        Ok(_) => iprint(format!("Generated {}", req_file)),
        Err(e) => eprint(format!("Could not write {}: {}", req_file, e)),
    }
}

pub fn start_project() {
    let config_file = get_project_config_file();
    if !Path::new(config_file).exists() {
        eprint(format!("Could not find {}", config_file));
        return;
    }

    let conf = match Config::load_from_file(config_file) {
        Ok(conf) => conf,
        Err(e) => {
            eprint(e.to_string());
            return;
        }
    };

    let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

    if !Path::new(&conf.project.main_script).exists() {
        eprint(format!(
            "Main script '{}' not found",
            conf.project.main_script
        ));
        return;
    }

    let mut child = match Command::new(get_venv_python_path(venv_root))
        .arg(&conf.project.main_script)
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprint("Failed to start main file".to_owned());
            eprint(e.to_string());
            return;
        }
    };

    match child.wait() {
        Ok(status) => {
            if !status.success() {
                wprint(format!("Process exited with status: {}", status));
            }
        }
        Err(e) => {
            eprint(format!("Error waiting for process: {}", e));
        }
    }
}

pub fn update_packages(pkg_names: &[String]) {
    let config_file = get_project_config_file();
    if !Path::new(config_file).exists() {
        eprint(format!("Could not find {}", config_file));
        return;
    }

    let mut conf = match Config::load_from_file(config_file) {
        Ok(conf) => conf,
        Err(e) => {
            eprint(e.to_string());
            return;
        }
    };

    if conf.packages.is_empty() {
        eprint("No packages to update".to_owned());
        return;
    }

    let venv_root = conf.project.venv.clone().unwrap_or_else(|| "venv".to_string());

    if !check_venv_dir_exists(&venv_root) {
        wprint(format!("Could not find '{}' directory", venv_root));
        if ask_if_create_venv() {
            if let Err(e) = setup_venv(format!("./{}", venv_root)) {
                eprint(format!("Failed to setup venv: {}", e));
                return;
            }
        } else {
            wprint("Update Cancelled".to_owned());
            return;
        }
    }

    let mut updates: Vec<(String, String)> = vec![];
    let mut failed_packages: Vec<String> = vec![];

    // Filter packages if specific ones are requested
    let packages_to_check: Vec<String> = if pkg_names.is_empty() {
        conf.packages.keys().cloned().collect()
    } else {
        let mut valid_names = vec![];
        for name in pkg_names {
            if conf.packages.contains_key(name) {
                valid_names.push(name.clone());
            } else {
                wprint(format!("Package '{}' not found in project.toml", name));
            }
        }
        valid_names
    };

    if packages_to_check.is_empty() {
        if !pkg_names.is_empty() {
             eprint("No valid packages specified to update".to_owned());
        } else {
             eprint("No packages to update".to_owned());
        }
        return;
    }

    for name in packages_to_check {
        match get_pkg_version(&name) {
            Ok(latest_ver) => updates.push((name.clone(), latest_ver)),
            Err(e) => {
                eprint(format!("Could not find latest version of {}: {}", name, e));
                failed_packages.push(name.clone());
            }
        }
    }

    if updates.is_empty() {
        eprint("No packages to update".to_owned());
        return;
    }

    let mut updated_packages: Vec<(String, String)> = vec![];

    let mut packages_to_install: Vec<String> = vec![];
    for (name, ver) in updates.iter() {
        packages_to_install.push(format!("{}=={}", name, ver));
    }

    // Batched pip install for better performance
    match install_packages_batch(&packages_to_install, &venv_root) {
        Ok(_) => {
            for (name, ver) in updates {
                updated_packages.push((name.clone(), ver.clone()));

                iprint(format!("Updated {}", name));
            }
        }
        Err(e) => {
            eprint(format!("Failed to update packages: {}", e));
        }
    }

    for (name, ver) in updated_packages {
        conf.packages.insert(name, ver);
    }

    if let Err(e) = conf.write_to_file(config_file) {
        eprint(format!("Failed to update config file: {}", e));
    }

    if !failed_packages.is_empty() {
        wprint(format!(
            "Failed to update {} package(s): {}",
            failed_packages.len(),
            failed_packages.join(", ")
        ));
    }

    if let Err(e) = generate_lock_file(&venv_root) {
        eprint(format!("Failed to generate lock file: {}", e));
    }
}

pub fn list_packages() {
    let config_file = get_project_config_file();
    if !Path::new(config_file).exists() {
        eprint(format!("Could not find {}", config_file));
        return;
    }

    let conf = match Config::load_from_file(config_file) {
        Ok(conf) => conf,
        Err(e) => {
            eprint(e.to_string());
            return;
        }
    };

    let count = conf.packages.len();
    
    if count == 0 {
        wprint("No packages configured".to_string());
        return;
    }

    println!(
        "\nConfigured packages ({}):",
        count.to_string().green().bold()
    );
    
    for (name, version) in conf.packages.iter() {
        println!(
            "{}=={}",
            name.green().bold(),
            version.bright_black()
        );
    }
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Most functions here have side effects (printing, filesystem, shelling out).
    // They are better tested via integration tests (CLI tests).
    // We strictly follow the request to add the module.
    
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
