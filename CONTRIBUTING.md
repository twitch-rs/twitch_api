# Contributing to twitch_api

## Git basics

### Cloning the repo

```sh
git clone https://github.com/twitch-rs/twitch_api.git --recurse-submodules
cd twitch_api
```

You can also use the [GitHub CLI](https://cli.github.com/) to clone the repository.

```sh
gh repo clone twitch-rs/twitch_api
cd twitch_api
```

### Checking out a branch

```sh
git checkout <branch_name>
```

to create a new branch (this is now default behaviour in newer git version)

```sh
git checkout -b <branch_name>
```

### Fetching the git submodules

To get started with using this repository, ensure you have initialized and updated the submodules in order to retrieve their contents.

```sh
git submodule update --init --recursive
```

alternatively, you can also run

```sh
git submodule init
git submodule update
```

### Resetting

```sh
# reset the last commit, keeping changes
git reset HEAD~1
# reset the last commit, discarding changes
git reset HEAD~1 --hard
```

### Rebasing

To rebase your branch on top of the latest changes on the main branch, run

```sh
git pull upstream main --rebase
```

if you want to combine changes into one commit, you can also use rebase to interactively squash commits

```sh
# Using relative commit
git rebase -i HEAD~<number_of_commits>
# Using a specific commit
git rebase -i <commit_hash>
```

## Creating a pull request

To create a pull request, see [GitHub's documentation](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

To make things easy, you can use the [GitHub CLI](https://cli.github.com/) to fork the repository, checkout issues and create pull requests from the command line.

```sh
# fork the repository, creates a upstream remote, can be used in an already cloned repository as well
gh repo fork twitch-rs/twitch_api --clone
# navigate into the fork
cd twitch_api

# sync submodules
git submodule update --init --recursive

# create a branch
git checkout -b <branch_name>

# checkout an issue and create a branch
gh issue develop -c <number/url>

# create a pull request
gh pr create

# rebase changes with main
git pull upstream main --rebase
```

### Maintaining

#### Updating twitch_oauth2

To update the `twitch_oauth2` submodule, run the appropriate
[`git submodule` command](https://git-scm.com/book/en/v2/Git-Tools-Submodules).
For example, to update to the latest commit on the remote default branch,
you may want to run:

```sh
git submodule update --remote twitch_oauth2
```
