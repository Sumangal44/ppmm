mod ppm_functions;
mod project_managers;
mod settings;
mod utils;

use clap::Parser;
use project_managers::Action;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

/// Python Project Manager
#[derive(Parser, Debug)]
#[clap(author=AUTHOR, version=VERSION, about=ABOUT, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Action,
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Action::New(project) => project.create_project(false),
        Action::Init(project) => project.create_project(true),
        Action::Add(add_proj) => add_proj.add_package(),
        Action::Rm(rp) => rp.remove_package(),
        Action::Run(run) => run.run_script(),
        Action::Install(installer) => installer.install_packages(),
        Action::Build(builder) => builder.build_project(),
        Action::Bump(bumper) => bumper.bump_version(),

        Action::Info => {
            ppm_functions::show_project_info();
            Ok(())
        }
        Action::Gen => {
            ppm_functions::gen_requirements();
            Ok(())
        }
        Action::Start => {
            ppm_functions::start_project();
            Ok(())
        }
        Action::Update(update) => update.update_package(),
        Action::List => ppm_functions::list_packages(),
        Action::Doctor => ppm_functions::doctor(),
    }
}