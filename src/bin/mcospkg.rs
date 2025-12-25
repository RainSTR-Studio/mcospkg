/// This file contains the executable file `mcospkg`.
///
/// # Usage
///
/// ```bash
/// mcospkg install [OPTION] <PACKAGES...>
/// mcospkg remove [OPTION] <PACKAGES...>
/// ```
///
/// # Explain
///
/// [OPTION]: Including the `-y`, `-h` and `-V`;
///
/// <PACKAGES> : The package you want to install/update/remove
///
/// This is the explaining of [OPTIONS]:
///
/// -y, --bypass: Install/remove/update packages WITHOUT asking;
/// -h, --help  : Get help message;
///
/// -V, --version: Print the version to the screen
///
/// # Example
///
/// ```bash
/// mcospkg install python  # Install package called "python"
/// mcospkg remove apt      # Remove the package called "apt"
/// mcospkg update          # Update all packages to the latest version
/// mcospkg update mcospkg  # Update the package "mcospkg" to the latest version
/// ```
///
/// For more information, type: `mcospkg -h`
//
// Now, we need to import some modules:
use clap::{Parser, Subcommand};
use colored::Colorize;
use mcospkg::VERSION;
use mcospkg::get_installed_package_info;
use mcospkg::{Color, INSTALL_DATA, REMOVE_DATA};

#[cfg(target_os = "linux")]
use is_root::is_root;
#[cfg(target_os = "linux")]
use std::process::exit;

// ========structs define area=========

// ====Arguments define area====
#[derive(Parser, Debug)]
#[command(name = "mcospkg")]
#[command(about = "A linux package-manager made for MinecraftOS (Main program)")]
#[command(version = VERSION)]

// Define argument lists
struct Args {
    #[command(subcommand)]
    operation: Operations,
}

// Define Subcommand
#[derive(Subcommand, Debug)]
enum Operations {
    #[command(about = "Install the package(s).")]
    Install {
        // Get packages
        #[arg(required = true, help = "The package(s) you want to install")]
        packages: Vec<String>,

        // And should we bypass asking
        #[arg(
            long = "bypass",
            short = 'y',
            default_value_t = false,
            help = "Specify it will not ask ANY questions"
        )]
        bypass_ask: bool,

        // And should we reinstall it
        #[arg(
            long = "reinstall",
            short = 'r',
            default_value_t = false,
            help = "Specify it will (re)install package"
        )]
        reinstall: bool,
    },

    // And remove
    #[command(about = "Remove the package(s).")]
    Remove {
        // Get packages
        #[arg(required = true, help = "The package(s) you want to remove")]
        packages: Vec<String>,

        // And should we bypass asking
        #[arg(
            long = "bypass",
            short = 'y',
            default_value_t = false,
            help = "Specify it will not ask ANY questions"
        )]
        bypass_ask: bool,
    },

    #[command(about = "List some available package(s)")]
    List {
        // Get is list installed packages only.
        #[arg(long = "installed", short = 'i', help = "List installed packages only")]
        list_installed_only: bool,
    },
}

// ========functions define area==========
fn main() {
    #[cfg(target_os = "linux")]
    let color = Color::new();

    // Parse arguments
    let args = Args::parse();

    // Make sure that the user is root. (only Linux)
    #[cfg(target_os = "linux")]
    if !is_root() {
        eprintln!(
            "{}: You must run this program with root privileges.",
            color.error
        );

        eprintln!(
            "{}: Did you forget to add \"sudo\" in front of the command? :)",
            color.tip
        );
        exit(1);
    }

    match args.operation {
        Operations::Install {
            packages,
            bypass_ask,
            reinstall,
        } => install(packages, bypass_ask, reinstall),
        Operations::Remove {
            packages,
            bypass_ask,
        } => remove(&packages, bypass_ask),
        Operations::List {
            list_installed_only,
        } => list(list_installed_only),
    };
}

fn install(pkglist: Vec<String>, bypass_ask: bool, reinstall: bool) {
    // Presets
    let color = Color::new();

    // Tell user is this mode is "reinstall"
    if reinstall {
        println!("{}: Reinstall mode has been enabled.", color.note);
    }

    // Init the InstallData struct
    let mut install_data = INSTALL_DATA.clone();

    // Stage 1: Get the pkgindex from the repositories
    install_data.step1_explain_pkg(&pkglist); // Stage 2: Check if the package is exist

    // Stage 2: Check the packages' dependencies
    install_data.step2_check_deps(&pkglist);

    // Stage 3: Check if package is installed
    install_data.step3_check_installed(reinstall);

    // Stage 4: Download the package
    install_data.step4_download(bypass_ask);

    // Stage 5: Check sha256sums integrity
    install_data.step5_check_sums();

    // Stage 6: Extract the package
    install_data.step6_extract();

    // Stage 7: Install the package
    install_data.step7_install();

    // And, that's complete!
}

// This is the install function
// =====S====P====L====I====T=====
// This is the remove function

fn remove(pkglist: &[String], bypass_ask: bool) {
    // Init the RemoveData struct
    let mut remove_data = REMOVE_DATA.clone();

    // Stage 1: Explain the package
    remove_data.step1_explain_pkg();

    // Stage 2: Check the dependencies
    remove_data.step2_check_deps(&pkglist);

    // Stage 3: Ask user
    remove_data.step3_ask_user(bypass_ask);

    // Stage 4: Remove the package
    remove_data.step4_remove();

    // Completed!!!!!!! :)
}

// This is the remove function
// =====S====P====L====I====T=====
// This is the list function
fn list(list_installed_only: bool) {
    // Let me see... It's very easy to print the installed
    // packages' information...
    //
    // So let's handle the installed packages first...
    println!("{}", "====Here are the installed packages====".bold());
    // First, get the installed packages info.
    let installed_pkg = get_installed_package_info();

    // This data type is HashMap<String, PkgInfoToml>,
    // the String is the package name.
    for (package, info) in &installed_pkg {
        let version = &info.version;
        println!(
            "{} {}, {} {}",
            "Package".bold().cyan(),
            package.bold(),
            "version".bold().cyan(),
            version.bold()
        );
    }

    // This won't be show if list_installed_only set to true.
    if !list_installed_only {
        //TODO: Get all packages from each repository
    }
}
