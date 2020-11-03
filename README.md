```bash
   ___ _                  _        __
  / __\ |__   __ _ _ __  (_) ___  / /  ___   __ _
 / /  | '_ \ / _' | '_ \ | |/ _ \/ /  / _ \ / _' |
/ /___| | | | (_| | | | || |  __/ /__| (_) | (_| |
\____/|_| |_|\__,_|_| |_|/ |\___\____/\___/ \__, |
                       |__/                 |___/

  v: $version - https://github.con/sayjeyhi/chanjelog

flag  |  description
-----------------------------------------------------
-h    |  show help
-r    |  *{string} git repo url e.g: git.com/project.git
-f    |  {string} a file path to set version init with -s search format e.g: 'docker-compose.yml'
-s    |  {string} search pattern in version file e.g: '- DEPLOY_VERSION=[0-9]*\.[0-9]*\.[0-9]*'
-p    |  {string} search replace pattern in version file e.g: '- DEPLOY_VERSION='
-u    |  {string} jira tasks prefix to automaticly convery #[0-9]* to jira links in commit messages e.g: 'https:\/\/jira.site.com\/browse\/SP-'
-g    |  {boolean} to control adding tag on git or not e.g: [1|0]
-v    |  {string} to control version increase type e.g: ['patch'|'minor'|'major'|'prere'|'build']
-c    |  {boolean} to control update changeslog.md file e.g: [1|0]


You should have CI_USER and CI_ACCESS_TOKEN variables to allow ci to update your git.
---
An open source hack by @sayjeyhi - https://sayjeyhi.com with SemVer2.0.0
```

### Getting started
Just download `index.sh` file and put it in your files, then you can run it in your CI flow
easily with pure bash, it will do all of `semver` managing and `changeslog` generating jobs
for you without any extra config!

### Features
As you can figure out from abow section and also in `chanjeslog -h`, we have:
- auto version detect
- auto connect to git repo
- auto commit messages combine between versions with author name and date of change
- semver support
- update a custom file, like `.env` or `docker.compose.yml` files to have version in app runtime
- task manager link update in commit messages
- good configurable params
- ..

