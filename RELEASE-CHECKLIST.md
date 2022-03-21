Release Checklist
-----------------
* Make branch `release-vx.y.z`
* Run `cargo update` and review changes. Commit updated lock-file.
* Run [`cargo outdated`](https://github.com/kbknapp/cargo-outdated), and review semver incompatible updates.
  Update every dependency unless there is a strong motivation to do otherwise.
  Update `Changelog.md` appropriately
* push the branch to upstream repository, and make a pr. Label as `release` and add milestone for this version. Body should be changelog body for this version
  * Github CLI (in powershell)
    ```pwsh
    gh pr create --web -p $(git branch --show-current | rg -e "release-v(.*)" -or 'v$1') -l release -b "$(rg --multiline-dotall -Um 1 -i CHANGELOG.md -e "## (\[v.*?\] - .*?\n[^$]*)\n## \[" -r '$1' | Select -SkipLast 1 | out-string)"
    ```
* Run [`cargo release --dry-run -vv [level=release]`](https://github.com/sunng87/cargo-release) to ensure everything looks correct.
* run `cargo release -vv [level=release]`,
  * This will update `CHANGELOG.md` and update crate version in all applicable places.
  * A commit will also be done, push it

* do bors r+ to merge.
* Checkout and pull main branch. Push to branch release.
* ```pwsh
* git checkout main; git pull
* git push <remote> main:release
* ```
* publish version to crates.io (skip verify if you want)
  ```
  cargo publish --no-verify
  ```
* Now, publish tag.
  * (in powershell)
    ```
    git tag -e --cleanup=verbatim -a $(rg -i Cargo.toml -e "^version = (.*)" -or 'v$1') -m "$(rg --multiline-dotall -Um 1 -i CHANGELOG.md -e "## (\[v.*?\] - .*?\n)## \[" -r '$1' | Select -SkipLast 1 | out-string)"
    ```
  * push tag with
    ```
    git push <remote> <tag>
    ```