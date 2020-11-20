use exitfailure::ExitFailure;
use std::path::PathBuf;
use structopt::StructOpt;

use dcli::error::Error;
use dcli::utils::{print_error, print_standard};
use dcli::manifestinterface::ManifestInterface;

#[derive(StructOpt)]
/// Command line tool for searching the Destiny 2 manifest
///
/// Command line tool for retrieving character information for specified member id
/// Retrieves character information for the specified member id.
struct Opt {
    ///Local path the Destiny 2 manifest database file.
    #[structopt(short = "m", long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,

    ///terse output in the form of class_name:character_id . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    _verbose: bool,

    ///Print out additional information for the API call
    #[structopt(long = "hash", required = true)]
    hash: u32,
}

async fn search_manifest_by_hash(hash: u32, manifest_path: PathBuf) -> Result<Vec<String>, Error> {
    let mut manifest = ManifestInterface::new(manifest_path, false).await?;
    let out = manifest.find(hash).await?;

    Ok(out)
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let results:Vec<String> = match search_manifest_by_hash(opt.hash, opt.manifest_path).await {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Could not search manifest : {:#}", e), !opt.terse);
            std::process::exit(1);
        }
    };

    if results.is_empty() {
        print_standard(&format!("No items found."), !opt.terse);
        std::process::exit(0);
    }

    for r in results.iter() {
        print_standard(r, true);
    }

    Ok(())
}