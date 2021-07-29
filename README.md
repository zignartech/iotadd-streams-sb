# api-iota-streams
### Usage
Build
```bash
cargo build --release
```
Use in bash
```bash
./target/release/api-iota-streams
```

# Conventional Commit tool and Jira Integration
## Using the command line tool

### If your repo is [Commitizen-friendly]:

Simply use `git cz` or just `cz` instead of `git commit` when committing. You can also use `git-cz`, which is an alias for `cz`.
## Making your repo Commitizen-friendly

For this example, we'll be setting up our repo to use [AngularJS's commit message convention](https://github.com/angular/angular.js/blob/master/DEVELOPERS.md#-git-commit-guidelines) also known as [conventional-changelog](https://github.com/ajoslin/conventional-changelog).

First, install the Commitizen cli tools:

```sh
npm install commitizen -g
```

Next, initialize your project to use the cz-conventional-changelog adapter by typing:

```sh
commitizen init cz-conventional-changelog --save-dev --save-exact
```

Or if you are using Yarn:

```sh
commitizen init cz-conventional-changelog --yarn --dev --exact
```
The above command does three things for you.

1. Installs the cz-conventional-changelog adapter npm module
2. Saves it to package.json's dependencies or devDependencies
3. Adds the `config.commitizen` key to the root of your **package.json** as shown here:

```json
...
  "config": {
    "commitizen": {
      "path": "cz-conventional-changelog"
    }
  }
```

Alternatively, commitizen configs may be added to a .czrc file:

```json
{
  "path": "cz-conventional-changelog"
}
```

This just tells Commitizen which adapter we actually want our contributors to use when they try to commit to this repo.
