use std::env;
use std::process::{Command, exit};

const VERSION: &str = "1.3.0";

fn main() {
    // Command-line arguments
    let args: Vec<String> = env::args().collect();

    // Helper variables
    let nat = "0|[1-9][0-9]*";
    let alphanum = "[0-9]*[A-Za-z-][0-9A-Za-z-]*";
    let ident = format!("{}|{}", nat, alphanum);
    let field = "[0-9A-Za-z-]+";
    let dir = env::current_dir().unwrap();
    let now = chrono::Local::now();
    let add_git_tag = true;
    let generate_changes_log = true;
    let version_bump_type = "patch";
    let semver_regex = format!(
        "^[vV]?({})\\.({})\\.({})(\\-({})(\\.({}))*)?(\\+{}(\\.{})*)?$",
        nat, nat, nat, ident, ident, field, field
    );

    // Validate supplied version
    fn validate_version(version: &str) -> Option<(String, String, String, String, String)> {
        let re = regex::Regex::new(&semver_regex).unwrap();
        if let Some(captures) = re.captures(version) {
            let major = captures.get(1).unwrap().as_str();
            let minor = captures.get(2).unwrap().as_str();
            let patch = captures.get(3).unwrap().as_str();
            let prere = captures.get(5).map_or("", |m| m.as_str());
            let build = captures.get(8).map_or("", |m| m.as_str());
            Some((major.into(), minor.into(), patch.into(), prere.into(), build.into()))
        } else {
            None
        }
    }

    // Generate new version
    fn generate_version(command: &str, sub_version: Option<&str>, version: &str) -> Option<String> {
        let parts = validate_version(version)?;
        let (major, minor, patch, prere, build) = (parts.0, parts.1, parts.2, parts.3, parts.4);

        let new = match command {
            "major" => format!("{}.0.0", major.parse::<u64>().unwrap() + 1),
            "minor" => format!("{}.{}.0", major, minor.parse::<u64>().unwrap() + 1),
            "patch" => format!("{}.{}.{}", major, minor, patch.parse::<u64>().unwrap() + 1),
            "release" => format!("{}.{}.{}", major, minor, patch),
            "prerel" => format!("{}.{}.{}-{}", major, minor, patch, sub_version.unwrap()),
            "build" => format!("{}.{}.{}{}+{}", major, minor, patch, prere, sub_version.unwrap()),
            _ => return None,
        };

        Some(new)
    }

    // Parse command-line arguments
    let mut version_change_file = None;
    let mut url_prefix = None;
    let mut add_git_tag = Some(true);
    let mut version_bump_type = Some("patch".to_string());
    let mut generate_changes_log = Some(true);
    let mut git_repo = None;
    let mut file_search_pattern = None;
    let mut file_replace_pattern = None;

    let mut idx = 1;
    while idx < args.len() {
        match args[idx].as_str() {
            "-f" => {
                version_change_file = Some(args[idx + 1].clone());
                idx += 2;
            }
            "-u" => {
                url_prefix = Some(args[idx + 1].clone());
                idx += 2;
            }
            "-g" => {
                add_git_tag = Some(args[idx + 1].parse::<bool>().unwrap());
                idx += 2;
            }
            "-v" => {
                version_bump_type = Some(args[idx + 1].clone());
                idx += 2;
            }
            "-c" => {
                generate_changes_log = Some(args[idx + 1].parse::<bool>().unwrap());
                idx += 2;
            }
            "-r" => {
                git_repo = Some(args[idx + 1].clone());
                idx += 2;
            }
            "-s" => {
                file_search_pattern = Some(args[idx + 1].clone());
                idx += 2;
            }
            "-p" => {
                file_replace_pattern = Some(args[idx + 1].clone());
                idx += 2;
            }
            _ => {
                println!("{}", usage());
                exit(1);
            }
        }
    }

    println!("{}", usage());
}

fn usage() -> String {
    format!(
        "{}\n   ___ _                  _        __\n  / __\\ |__   __ _ _ __  (_) ___  / /  ___   __ _\n / /  | '_ \\ / _' | '_ \\ | |/ _ \\/ /  / _ \\ / _' |\n/ /___| | | | (_| | | | || |  __/ /__| (_) | (_| |\n\\____/|_| |_|\\__,_|_| |_|/ |\\___\\____/\\___/ \\__, |\n                       |__/                 |___/\n\n  v: {} - https://github.com/sayjeyhi/chanjelog\n\nflag  |  description\n-----------------------------------------------------\n-h    |  show help\n-r    |  *{string} git repo url e.g: git.com/project.git\n-f    |  {string} a file path to set version init with -s search format e.g: 'docker-compose.yml'\n-s    |  {string} search pattern in version file e.g: '- DEPLOY_VERSION=[0-9]*\\.[0-9]*\\.[0-9]*'\n-p    |  {string} search replace pattern in version file e.g: '- DEPLOY_VERSION='\n-u    |  {string} jira tasks prefix to automaticly convery #[0-9]* to jira links in commit messages e.g: 'https://jira.site.com/browse/SP-'\n-g    |  {boolean} to control adding tag on git or not e.g: [1|0]\n-v    |  {string} to control version increase type e.g: ['patch'|'minor'|'major'|'prere'|'build']\n-c    |  {boolean} to control update changeslog.md file e.g: [1|0]\n\nYou should have CI_USER and CI_ACCESS_TOKEN variables to allow ci to update your git.\n---\nAn open source hack by @sayjeyhi - https://sayjeyhi.com with SemVer2.0.0\n",
        VERSION
    )
}
