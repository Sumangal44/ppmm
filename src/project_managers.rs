use crate::settings::*;
use crate::utils::*;
use clap::{Args, Subcommand};
use colored::Colorize;
use std::{collections::HashMap, fs, path::Path, process::Command, time::Instant};

const STARTER_SOURCE_PY: &str = "
def main():
    print('Hello From PPMM!')
    print('Happy Coding!') 

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
    /// Update all or specific packages to their latest versions
    Update(UpdatePackage),
    /// Build the project
    Build(BuildProject),
    /// Bump project version (major, minor, patch)
    Bump(BumpVersion),
    /// List packages declared in project.toml
    List,
    /// Dependency lock and security commands powered by ppmm-lock
    Lock(LockCommand),
    Doctor,
}

#[derive(Args, Debug)]
pub struct LockCommand {
    #[clap(subcommand)]
    pub command: LockAction,
}

#[derive(Subcommand, Debug)]
pub enum LockAction {
    /// Resolve and pin all dependencies
    Lock(LockOptions),
    /// Install from lock or requirements file
    Install(LockInstallOptions),
    /// Check for available upgrades
    Update(LockUpdateOptions),
    /// Scan for known vulnerabilities
    Audit(LockAuditOptions),
}

#[derive(Args, Debug)]
pub struct LockOptions {
    /// Path to requirements file
    #[clap(short = 'r', long = "requirements", default_value = "requirements.txt")]
    pub requirements: String,
    /// Skip hash computation
    #[clap(long = "no-hashes", takes_value = false)]
    pub no_hashes: bool,
}

#[derive(Args, Debug)]
pub struct LockInstallOptions {
    /// Explicit requirements file to install from
    #[clap(short = 'r', long = "requirements")]
    pub requirements: Option<String>,
}

#[derive(Args, Debug)]
pub struct LockUpdateOptions {
    /// Apply updates and regenerate lockfile
    #[clap(long = "apply", takes_value = false)]
    pub apply: bool,
}

#[derive(Args, Debug)]
pub struct LockAuditOptions {
    /// Output results as JSON
    #[clap(long = "json-output", takes_value = false)]
    pub json_output: bool,
}

impl LockCommand {
    pub fn execute(&self) -> Result<(), String> {
        match &self.command {
            LockAction::Lock(options) => {
                let mut args: Vec<String> = vec!["lock".to_string()];
                args.push("--requirements".to_string());
                args.push(options.requirements.clone());
                if options.no_hashes {
                    args.push("--no-hashes".to_string());
                }
                run_ppmm_lock(&args)
            }
            LockAction::Install(options) => {
                let mut args: Vec<String> = vec!["install".to_string()];
                if let Some(req) = &options.requirements {
                    args.push("--requirements".to_string());
                    args.push(req.clone());
                }
                run_ppmm_lock(&args)
            }
            LockAction::Update(options) => {
                let mut args: Vec<String> = vec!["update".to_string()];
                if options.apply {
                    args.push("--apply".to_string());
                }
                run_ppmm_lock(&args)
            }
            LockAction::Audit(options) => {
                let mut args: Vec<String> = vec!["audit".to_string()];
                if options.json_output {
                    args.push("--json-output".to_string());
                }
                run_ppmm_lock(&args)
            }
        }
    }
}

pub struct ProjectCreator {
    project: ProjectConf,
    is_init: bool,
}

impl ProjectCreator {
    fn new(project: ProjectConf, is_init: bool) -> ProjectCreator {
        ProjectCreator { project, is_init }
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

        let path = if self.is_init {
            "."
        } else {
            &self.project.name
        };
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
                if self.is_init {
                    "./main.py"
                } else {
                    "./src/main.py"
                }
                .to_string(),
                self.project.venv.clone(),
            ),
            HashMap::new(),
            HashMap::new(),
        );
        conf.scripts.insert(
            "upgrade-pip".to_string(),
            "python -m pip install --upgrade pip".to_string(),
        );

        let config_path = self.get_path_with(get_project_config_file());
        conf.write_to_file(&config_path)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }

    pub fn create_project(&self) -> Result<(), String> {
        let start = Instant::now();
        let proj_dest = self.get_path_with("src");

        if project_exists(&self.project.name, self.is_init) {
            return Err(format!(
                "Project With Name '{}' Already Exists",
                &self.project.name
            ));
        }

        fs::create_dir_all(&proj_dest).map_err(|e| format!("Failed to create directory: {}", e))?;

        self.create_boilerplate_files()?;

        self.create_git()?;

        if !self.project.no_venv {
            let venv_path = self
                .project
                .venv
                .clone()
                .unwrap_or_else(|| "venv".to_string());
            setup_venv(self.get_path_with(&venv_path))
                .map_err(|e| format!("Failed to setup venv: {}", e))?;
        } else {
            wprint("Virtual environment is disabled, some commands might not work".to_string());
        }

        self.save_config()?;

        let elapsed = start.elapsed();
        iprint(format!("{} in {}s", "Completed".green(), elapsed.as_secs()));
        println!("\nTo get started:");
        if !self.is_init {
            println!("  cd {}", self.project.name.blue());
        }
        println!("  {} start\n", "ppm".red());

        Ok(())
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
    /// Set Virtual Environment Name
    #[clap(long = "venv")]
    venv: Option<String>,
    /// Enable Git
    #[clap(short = 'g', long = "git", takes_value = false)]
    git: bool,
    /// Don't Create Virtual Environment
    #[clap(short = 'e', long = "no-venv", takes_value = false)]
    no_venv: bool,
}

impl ProjectConf {
    pub fn create_project(&self, is_init: bool) -> Result<(), String> {
        let proj_creator = ProjectCreator::new(self.clone(), is_init);
        proj_creator.create_project()
    }
}

#[derive(Args, Debug)]
pub struct AddPackage {
    /// List of packages to add
    pub pkg_names: Vec<String>,
}

impl AddPackage {
    pub fn add_package(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let mut conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

        if self.pkg_names.is_empty() {
            wprint("No packages specified".to_string());
            return Ok(());
        }

        match install_packages_batch(&self.pkg_names, venv_root) {
            Ok(_) => {
                for pkg_name in self.pkg_names.iter() {
                    let (vname, ver) = parse_version(pkg_name);
                    let version = match ver {
                        Some(v) => v,
                        None => match get_pkg_version(&vname) {
                            Ok(v) => v,
                            Err(e) => {
                                eprint(format!("Failed to get version for '{}': {}", vname, e));
                                continue;
                            }
                        },
                    };

                    conf.packages.insert(vname.clone(), version);
                    iprint(format!("Package '{}' added successfully", &vname));
                }

                match conf.write_to_file(config_file) {
                    Ok(_) => {
                        if let Err(e) = generate_lock_file(venv_root) {
                            eprint(format!("Failed to generate lock file: {}", e));
                        }
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }

                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Args, Debug)]
pub struct RemovePackage {
    /// List of packages to remove
    pub pkg_names: Vec<String>,
}

impl RemovePackage {
    fn uninstall_package(&self, pkg: &str, venv_root: &str) -> Result<(), String> {
        if !check_venv_dir_exists(venv_root) {
            return Err("Virtual Environment Not Found".to_string());
        }

        iprint(format!("Uninstalling {}", pkg));
        let output = Command::new(get_venv_pip_path(venv_root))
            .arg("uninstall")
            .arg("-y")
            .arg(pkg)
            .output()
            .map_err(|e| format!("Failed to execute pip: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to uninstall: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    pub fn remove_package(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let mut conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let venv_root = conf
            .project
            .venv
            .clone()
            .unwrap_or_else(|| "venv".to_string());

        for pkg_name in self.pkg_names.iter() {
            if !conf.packages.contains_key(pkg_name) {
                eprint(format!("Package '{}' does not exist", pkg_name));
                continue;
            }

            match self.uninstall_package(pkg_name, &venv_root) {
                Ok(_) => {
                    conf.packages.remove(pkg_name);
                    match conf.write_to_file(config_file) {
                        Ok(_) => {
                            iprint(format!("Package '{}' removed successfully", pkg_name));
                            if let Err(e) = generate_lock_file(&venv_root) {
                                eprint(format!("Failed to generate lock file: {}", e));
                            }
                        }
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    return Err(format!("Failed to remove '{}': {}", pkg_name, e));
                }
            }
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct RunScript {
    /// Script Name
    pub script_name: String,
}

impl RunScript {
    pub fn run_script(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let cmd_str = match conf.scripts.get(&self.script_name) {
            Some(cmd) => cmd,
            None => {
                return Err(format!(
                    "Script with name '{}' does not exist",
                    self.script_name
                ));
            }
        };

        let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        } else {
            return Err("Unsupported OS".to_owned());
        };

        let current_path = std::env::var("PATH").unwrap_or_default();
        let separator = if cfg!(target_os = "windows") {
            ";"
        } else {
            ":"
        };
        let new_path = format!(
            "{}{}{}",
            get_venv_bin_dir(venv_root),
            separator,
            current_path
        );
        cmd.env("PATH", new_path);
        cmd.arg(cmd_str);

        match cmd.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    return Err(format!("Error waiting for script: {}", e));
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct Installer {
    /// Install from requirements
    #[clap(short = 'r', long = "requirements", default_value = "")]
    pub requirements: String,
}

impl Installer {
    fn install_from_req(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let mut conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let venv_root = conf
            .project
            .venv
            .clone()
            .unwrap_or_else(|| "venv".to_string());

        if !check_venv_dir_exists(&venv_root) {
            wprint(format!("Could not find '{}' directory", venv_root));
            if ask_if_create_venv() {
                if let Err(e) = setup_venv(format!("./{}", venv_root)) {
                    return Err(format!("Failed to setup venv: {}", e));
                }
            } else {
                wprint("Installation Cancelled".to_owned());
                return Ok(());
            }
        }

        let req_file = match fs::read_to_string(&self.requirements) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("Failed to read {}: {}", self.requirements, e));
            }
        };

        let pkg_names: Vec<&str> = req_file
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
            .collect();

        if pkg_names.is_empty() {
            wprint("No packages found in requirements file".to_owned());
            return Ok(());
        }

        let pkg_names_string: Vec<String> = pkg_names.iter().map(|&s| s.to_string()).collect();

        match install_packages_batch(&pkg_names_string, &venv_root) {
            Ok(_) => {
                for pkg_name in pkg_names {
                    let (vname, ver) = parse_version(pkg_name);
                    let version = match ver {
                        Some(v) => v,
                        None => match get_pkg_version(&vname) {
                            Ok(v) => v,
                            Err(e) => {
                                eprint(format!("Failed to get version for '{}': {}", vname, e));
                                continue;
                            }
                        },
                    };

                    conf.packages.insert(vname.clone(), version);
                    iprint(format!("Package '{}' installed successfully", &vname));
                }

                match conf.write_to_file(config_file) {
                    Ok(_) => {}
                    Err(e) => return Err(e.to_string()),
                }
            }
            Err(e) => {
                return Err(e);
            }
        }

        if let Err(e) = generate_lock_file(&venv_root) {
            eprint(format!("Failed to generate lock file: {}", e));
        }

        Ok(())
    }

    pub fn install_packages(&self) -> Result<(), String> {
        if !self.requirements.is_empty() {
            return self.install_from_req();
        }

        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        if conf.packages.is_empty() {
            wprint("No packages to install".to_owned());
            return Ok(());
        }

        let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

        if !check_venv_dir_exists(venv_root) {
            wprint(format!("Could not find '{}' directory", venv_root));
            if ask_if_create_venv() {
                if let Err(e) = setup_venv(format!("./{}", venv_root)) {
                    return Err(format!("Failed to setup venv: {}", e));
                }
            } else {
                wprint("Installation Cancelled".to_owned());
                return Ok(());
            }
        }

        if Path::new("ppmm.lock").exists() {
            iprint("Found ppmm.lock, installing from lock file...".to_string());
            let output = Command::new(get_venv_pip_path(venv_root))
                .arg("install")
                .arg("-r")
                .arg("ppmm.lock")
                .output();

            match output {
                Ok(out) => {
                    if !out.status.success() {
                        return Err(format!(
                            "Failed to install from lock file: {}",
                            String::from_utf8_lossy(&out.stderr)
                        ));
                    } else {
                        println!("{}", String::from_utf8_lossy(&out.stdout));
                        iprint("Installed from ppmm.lock successfully".to_string());
                        return Ok(());
                    }
                }
                Err(e) => {
                    return Err(format!("Failed to execute pip: {}", e));
                }
            }
        }

        let mut packages_to_install: Vec<String> = vec![];
        for (name, version) in conf.packages.iter() {
            packages_to_install.push(format!("{}=={}", name, version));
        }

        match install_packages_batch(&packages_to_install, venv_root) {
            Ok(_) => {
                for (name, _) in conf.packages.iter() {
                    iprint(format!("Package '{}' installed", name));
                }
            }
            Err(e) => return Err(format!("Failed to install packages: {}", e)),
        }

        if let Err(e) = generate_lock_file(venv_root) {
            eprint(format!("Failed to generate lock file: {}", e));
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct BuildProject;

impl BuildProject {
    pub fn build_project(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Check if build script exists
        let build_script = match conf.scripts.get("build") {
            Some(script) => script,
            None => {
                wprint("No 'build' script defined in project.toml".to_string());
                wprint("Add a [scripts] section with 'build = \"your build command\"'".to_string());
                return Ok(());
            }
        };

        iprint(format!("Building project: {}", conf.project.name));

        let venv_root = conf.project.venv.as_deref().unwrap_or("venv");

        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C");
            c
        } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            let mut c = Command::new("sh");
            c.arg("-c");
            c
        } else {
            return Err("Unsupported OS".to_owned());
        };

        let current_path = std::env::var("PATH").unwrap_or_default();
        let separator = if cfg!(target_os = "windows") {
            ";"
        } else {
            ":"
        };
        let new_path = format!(
            "{}{}{}",
            get_venv_bin_dir(venv_root),
            separator,
            current_path
        );
        cmd.env("PATH", new_path);
        cmd.arg(build_script);

        match cmd.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    return Err(format!("Error waiting for build script: {}", e));
                }
                iprint("Build completed successfully".to_string());
            }
            Err(e) => {
                return Err(format!("Failed to execute build script: {}", e));
            }
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct BumpVersion {
    /// Version bump type: major, minor, or patch
    #[clap(value_parser = ["major", "minor", "patch"])]
    pub bump_type: String,
}

impl BumpVersion {
    pub fn bump_version(&self) -> Result<(), String> {
        let config_file = get_project_config_file();
        if !Path::new(config_file).exists() {
            return Err(format!("Could not find {}", config_file));
        }

        let mut conf = match Config::load_from_file(config_file) {
            Ok(conf) => conf,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let current_version = conf.project.version.clone();
        let new_version = match bump_semantic_version(&current_version, &self.bump_type) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Failed to bump version: {}", e));
            }
        };

        conf.project.version = new_version.clone();

        match conf.write_to_file(config_file) {
            Ok(_) => {
                iprint(format!(
                    "Version bumped: {} → {}",
                    current_version.bright_cyan(),
                    new_version.bright_green()
                ));
            }
            Err(e) => {
                return Err(format!("Failed to update project.toml: {}", e));
            }
        }

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct UpdatePackage {
    /// List of packages to update
    pub pkg_names: Vec<String>,
}

impl UpdatePackage {
    pub fn update_package(&self) -> Result<(), String> {
        crate::ppm_functions::update_packages(self.pkg_names.as_slice());
        Ok(())
    }
}

/// Bump semantic version (major.minor.patch)
fn bump_semantic_version(version: &str, bump_type: &str) -> Result<String, String> {
    // Remove alpha/beta suffixes
    let base_version = version.split('-').next().unwrap_or(version);

    let parts: Vec<&str> = base_version.split('.').collect();
    if parts.len() != 3 {
        return Err(format!(
            "Invalid version format: {}. Expected major.minor.patch",
            version
        ));
    }

    let major: u32 = parts[0]
        .parse()
        .map_err(|_| format!("Invalid major version: {}", parts[0]))?;
    let minor: u32 = parts[1]
        .parse()
        .map_err(|_| format!("Invalid minor version: {}", parts[1]))?;
    let patch: u32 = parts[2]
        .parse()
        .map_err(|_| format!("Invalid patch version: {}", parts[2]))?;

    let new_version = match bump_type {
        "major" => format!("{}.0.0", major + 1),
        "minor" => format!("{}.{}.0", major, minor + 1),
        "patch" => format!("{}.{}.{}", major, minor, patch + 1),
        _ => {
            return Err(format!(
                "Unknown bump type: {}. Use 'major', 'minor', or 'patch'",
                bump_type
            ));
        }
    };

    Ok(new_version)
}
