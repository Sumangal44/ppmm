use clap::{Subcommand, Args};
use colored::Colorize;
use std::{fs, process::Command, path::Path, time::Instant, collections::HashMap};
use crate::utils::*;
use crate::settings::*;

const STARTER_SOURCE_PY: &str = "
def main():
    print('Hello From PPM!')

if __name__ == '__main__':
    main()
";

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Create New Project With Given Name
    New(ProjectConf),
    /// Initialize Project In Current Directory
    Init(ProjectConf),
    /// Add new packages to project 
    Add(AddPackage),
    /// Remove packages from project
    Rm(RemovePackage),
    /// Run a script defined in project.toml
    Run(RunScript),
    /// Install packages from project.toml or provided requirements.txt
    Install(Installer),
    /// Run main script defined in project.toml
    Start,
    /// Generate requirements.txt file
    Gen,
    /// Show the project.toml file
    Info,
    /// Update all packages 
    Update,
    /// Build the project
    Build(BuildProject),
    /// Bump project version (major, minor, patch)
    Bump(BumpVersion),
}

pub struct ProjectCreator {
    project: ProjectConf,
    is_init: bool,
}

impl ProjectCreator {
    fn new(project: ProjectConf, is_init: bool) -> ProjectCreator {
        ProjectCreator {
            project,
            is_init,
        }
    }

    fn get_path_with(&self, path: &str) -> String {
        if self.is_init {
            path.to_string()
        } else {
            format!("{}/{}", self.project.name, path)
        }
    }

    fn create_git(&self) -> Result<(), String> {
        if !self.project.git {
            return Ok(());
        }
        
        let path = if self.is_init { "." } else { &self.project.name };
        Command::new("git")
            .arg("init")
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to initialize git: {}", e))?;
        
        let gitignore_path = self.get_path_with(".gitignore");
        fs::write(&gitignore_path, "/build\n/venv\n")
            .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
        
        Ok(())
    }
    
    fn create_boilerplate_files(&self) -> Result<(), String> {
        let proj_dest = self.get_path_with("src");
        let main_file_path = format!("{}/main.py", proj_dest);
        fs::write(&main_file_path, STARTER_SOURCE_PY)
            .map_err(|e| format!("Failed to create main.py: {}", e))?;
        Ok(())
    }

    fn save_config(&self) -> Result<(), String> {
        let mut conf = Config::new(
            Project::new(
                self.project.name.clone(),
                self.project.version.clone(),
                self.project.description.clone(),
                if self.is_init { "./main.py" } else { "./src/main.py" }.to_string()  
            ),
            HashMap::new(),
            HashMap::new(),
        );
        conf.scripts.insert(
            "upgrade-pip".to_string(), 
            "python -m pip install --upgrade pip".to_string()
        );

        let config_path = self.get_path_with(get_project_config_file());
        conf.write_to_file(&config_path)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    pub fn create_project(&self) {
        let start = Instant::now();
        let proj_dest = self.get_path_with("src");
        
        if project_exists(&self.project.name, self.is_init) {
            eprint(format!("Project With Name '{}' Already Exists", &self.project.name));
            return;
        }
        
        if let Err(e) = fs::create_dir_all(&proj_dest) {
            eprint(format!("Failed to create directory: {}", e));
            return;
        }
        
        if let Err(e) = self.create_boilerplate_files() {
            eprint(e);
            return;
        }

        if let Err(e) = self.create_git() {
            eprint(e);
            return;
        }

        if !self.project.no_venv {
            if let Err(e) = setup_venv(self.get_path_with("venv")) {
                eprint(format!("Failed to setup venv: {}", e));
                return;
            }
        } else {
            wprint("Virtual environment is disabled, some commands might not work".to_string());
        }

        if let Err(e) = self.save_config() {
            eprint(e);
            return;
        }

        let elapsed = start.elapsed();
        iprint(format!("{} in {}s", "Completed".green(), elapsed.as_secs()));
        println!("\nTo get started:");
        if !self.is_init {
            println!("  cd {}", self.project.name.blue());
        }
        println!("  {} start\n", "ppm".red());
    }
}

#[derive(Args, Debug, Clone)]
pub struct ProjectConf {
    /// Set Project Name
    name: String,
    /// Set Project Version
    #[clap(short = 'v', long = "version", default_value = "0.1.0")]
    version: String,
    /// Set Project Description
    #[clap(short = 'd', long = "description", default_value = "")]
    description: String,
    /// Enable Git
    #[clap(short = 'g', long = "git", takes_value = false)]
    git: bool,
    /// Don't Create Virtual Environment
    #[clap(short = 'e', long = "no-venv", takes_value = false)]
    no_venv: bool,
}

impl ProjectConf {
    pub fn create_project(&self, is_init: bool) {
        let proj_creator = ProjectCreator::new(self.clone(), is_init);
        proj_creator.create_project();
    }
}

#[derive(Args, Debug)]
pub struct AddPackage {
    /// List of packages to add
    pub pkg_names: Vec<String>,
}

impl AddPackage {
    pub fn add_package(&self) {
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

        for pkg_name in self.pkg_names.iter() {
            let (vname, ver) = parse_version(pkg_name);

            match install_package(pkg_name) {
                Ok(_) => {
                    let version = match ver {
                        Some(v) => v,
                        None => {
                            match get_pkg_version(&vname) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprint(format!("Failed to get version for '{}': {}", vname, e));
                                    continue;
                                }
                            }
                        }
                    };
                    
                    conf.packages.insert(vname.clone(), version);
                    match conf.write_to_file(config_file) {
                        Ok(_) => iprint(format!("Package '{}' added successfully", &vname)),
                        Err(e) => {
                            eprint(e.to_string());
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprint(format!("Package '{}' failed to install: {}", &vname, e));
                }
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct RemovePackage {
    /// List of packages to remove
    pub pkg_names: Vec<String>,
}

impl RemovePackage {
    fn uninstall_package(&self, pkg: &str) -> Result<(), String> {
        if !check_venv_dir_exists() {
            return Err("Virtual Environment Not Found".to_string());
        }
        
        iprint(format!("Uninstalling {}", pkg));
        let output = Command::new(get_venv_pip_path())
            .arg("uninstall")
            .arg("-y")
            .arg(pkg)
            .output()
            .map_err(|e| format!("Failed to execute pip: {}", e))?;
        
        if !output.status.success() {
            return Err(format!("Failed to uninstall: {}", 
                              String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    pub fn remove_package(&self) {
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
        
        for pkg_name in self.pkg_names.iter() {
            if !conf.packages.contains_key(pkg_name) {
                eprint(format!("Package '{}' does not exist", pkg_name));
                continue;
            }
            
            match self.uninstall_package(pkg_name) {
                Ok(_) => {
                    conf.packages.remove(pkg_name);
                    match conf.write_to_file(config_file) {
                        Ok(_) => iprint(format!("Package '{}' removed successfully", pkg_name)),
                        Err(e) => {
                            eprint(e.to_string());
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprint(format!("Failed to remove '{}': {}", pkg_name, e));
                }
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct RunScript {
    /// Script Name
    pub script_name: String,
}

impl RunScript {
    pub fn run_script(&self) {
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

        let cmd_str = match conf.scripts.get(&self.script_name) {
            Some(cmd) => cmd,
            None => {
                eprint(format!("Script with name '{}' does not exist", self.script_name));
                return;
            }
        };

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        } else {
            eprint("Unsupported OS".to_owned());
            return;
        };
        
        cmd.env("PATH", get_venv_bin_dir());
        cmd.arg(cmd_str);

        match cmd.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprint(format!("Error waiting for script: {}", e));
                }
            }
            Err(e) => {
                eprint(e.to_string());
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct Installer {
    /// Install from requirements
    #[clap(short = 'r', long = "requirements", default_value = "")]
    pub requirements: String,
}

impl Installer {
    fn install_from_req(&self) {
        if !check_venv_dir_exists() {
            wprint("Could not find venv directory".to_owned());
            if ask_if_create_venv() {
                if let Err(e) = setup_venv("./venv".to_owned()) {
                    eprint(format!("Failed to setup venv: {}", e));
                    return;
                }
            } else {
                wprint("Installation Cancelled".to_owned());
                return;
            }
        }

        let req_file = match fs::read_to_string(&self.requirements) {
            Ok(f) => f,
            Err(e) => {
                eprint(format!("Failed to read {}: {}", self.requirements, e));
                return;
            }
        };

        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            eprint(format!("Could not find {}", config_file));
            return;
        }

        let pkg_names: Vec<&str> = req_file
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .collect();

        if pkg_names.is_empty() {
            wprint("No packages found in requirements file".to_owned());
            return;
        }

        let mut conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                eprint(e.to_string());
                return;
            }
        };

        for pkg_name in pkg_names.iter() {
            let (vname, ver) = parse_version(pkg_name);

            match install_package(pkg_name) {
                Ok(_) => {
                    let version = match ver {
                        Some(v) => v,
                        None => {
                            match get_pkg_version(&vname) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprint(format!("Failed to get version for '{}': {}", vname, e));
                                    continue;
                                }
                            }
                        }
                    };
                    
                    conf.packages.insert(vname.clone(), version);
                    match conf.write_to_file(config_file) {
                        Ok(_) => iprint(format!("Package '{}' installed successfully", &vname)),
                        Err(e) => {
                            eprint(e.to_string());
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprint(format!("Package '{}' failed to install: {}", &vname, e));
                }
            }
        }
    }

    pub fn install_packages(&self) {
        if !self.requirements.is_empty() {
            self.install_from_req();
            return;
        }

        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            eprint(format!("Could not find {}", config_file));
            return;
        }

        let  conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                eprint(e.to_string());
                return;
            }
        };

        if conf.packages.is_empty() {
            wprint("No packages to install".to_owned());
            return;
        }

        if !check_venv_dir_exists() {
            wprint("Could not find venv directory".to_owned());
            if ask_if_create_venv() {
                if let Err(e) = setup_venv("./venv".to_owned()) {
                    eprint(format!("Failed to setup venv: {}", e));
                    return;
                }
            } else {
                wprint("Installation Cancelled".to_owned());
                return;
            }
        }

        for (name, version) in conf.packages.iter() {
            let package_spec = format!("{}=={}", name, version);
            match install_package(&package_spec) {
                Ok(_) => iprint(format!("Package '{}' installed", name)),
                Err(e) => eprint(format!("Failed to install '{}': {}", name, e)),
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct BuildProject;

impl BuildProject {
    pub fn build_project(&self) {
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

        // Check if build script exists
        let build_script = match conf.scripts.get("build") {
            Some(script) => script,
            None => {
                wprint("No 'build' script defined in project.toml".to_string());
                wprint("Add a [scripts] section with 'build = \"your build command\"'".to_string());
                return;
            }
        };

        iprint(format!("Building project: {}", conf.project.name));
        
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        } else {
            eprint("Unsupported OS".to_owned());
            return;
        };
        
        cmd.env("PATH", get_venv_bin_dir());
        cmd.arg(build_script);

        match cmd.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprint(format!("Error waiting for build script: {}", e));
                    return;
                }
                iprint("Build completed successfully".to_string());
            }
            Err(e) => {
                eprint(format!("Failed to execute build script: {}", e));
            }
        }
    }
}

#[derive(Args, Debug)]
pub struct BumpVersion {
    /// Version bump type: major, minor, or patch
    #[clap(value_parser = ["major", "minor", "patch"])]
    pub bump_type: String,
}

impl BumpVersion {
    pub fn bump_version(&self) {
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

        let current_version = conf.project.version.clone();
        let new_version = match bump_semantic_version(&current_version, &self.bump_type) {
            Ok(v) => v,
            Err(e) => {
                eprint(format!("Failed to bump version: {}", e));
                return;
            }
        };

        conf.project.version = new_version.clone();

        match conf.write_to_file(config_file) {
            Ok(_) => {
                iprint(format!("Version bumped: {} → {}", current_version.bright_cyan(), new_version.bright_green()));
            }
            Err(e) => {
                eprint(format!("Failed to update project.toml: {}", e));
            }
        }
    }
}

/// Bump semantic version (major.minor.patch)
fn bump_semantic_version(version: &str, bump_type: &str) -> Result<String, String> {
    // Remove alpha/beta suffixes
    let base_version = version.split('-').next().unwrap_or(version);
    
    let parts: Vec<&str> = base_version.split('.').collect();
    if parts.len() != 3 {
        return Err(format!("Invalid version format: {}. Expected major.minor.patch", version));
    }

    let major: u32 = parts[0].parse()
        .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
    let minor: u32 = parts[1].parse()
        .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;
    let patch: u32 = parts[2].parse()
        .map_err(|_| format!("Invalid patch version: {}", parts[2]))?;

    let new_version = match bump_type {
        "major" => format!("{}.0.0", major + 1),
        "minor" => format!("{}.{}.0", major, minor + 1),
        "patch" => format!("{}.{}.{}", major, minor, patch + 1),
        _ => return Err(format!("Unknown bump type: {}. Use 'major', 'minor', or 'patch'", bump_type)),
    };

    Ok(new_version)
}