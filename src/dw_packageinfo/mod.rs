use crate::prelude::*;
use clap::ArgMatches;

pub fn run(args: &ArgMatches) -> DwResult<()> {
    init_logger(args);

    let filename = args
        .get_one::<String>("input")
        .ok_or_else(|| DwError::BadArguments("--input needed".to_string()))?;
    let package = Package::open(filename)?;
    println!("{package}");

    let manifest = package.manifest().expect("Android manifest");
    println!(
        " - package name: {}",
        manifest.package()?.expect("application package name")
    );
    println!(
        " - version code: {}, version name: {}",
        manifest.version_code()?.expect("application version code"),
        manifest.version_name()?.expect("application version name"),
    );
    println!(" - uses-permissions:");
    for permission in &manifest.uses_permissions(package.resources())? {
        match permission.name() {
            Some(name) => println!("   - {}", name),
            None => println!("   - Permission name not found"),
        }
    }
    println!(" - uses-features:");
    for feature in &manifest.uses_features(package.resources())? {
        match feature.name() {
            Some(name) => println!("   - {}", name),
            None => println!("   - Feature name not found"),
        }
    }
    println!(" - activities:");
    for activity in &manifest.activities(package.resources())? {
        match activity.name() {
            Some(name) => println!("   - {}", name),
            None => println!("   - Activity name not found"),
        }
    }
    println!(" - services:");
    for service in &manifest.services(package.resources())? {
        match service.name() {
            Some(name) => println!("   - {}",name),
            None => println!("   - Service name not found"),
        }
    }
    println!(" - receivers:");
    for receiver in &manifest.receivers(package.resources())? {
        match receiver.name() {
            Some(name) => println!("   - {}", name),
            None => println!("   - Receiver name not found"),
        }
    }
    println!(" - providers:");
    for provider in &manifest.providers(package.resources())? {
        match provider.name() {
            Some(name) => println!("   - {}", name),
            None => println!("   - Provider name not found"),
        }
    }
    Ok(())
}
