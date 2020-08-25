# Contributing to twitch_api2
[contributing-to-twitch-api2]: #contributing-to-twitch-api2


#### Synchronizing a subtree

There are two synchronization directions: `subtree push` and `subtree pull`.

```
git subtree push -P twitch_oauth2 git@github.com:your-github-name/twitch_oauth2 sync-from-api
```

takes all the changes that
happened to the copy in this repo and creates commits on the remote repo that match the local
changes. Every local commit that touched the subtree causes a commit on the remote repo, but is
modified to move the files from the specified directory to the external repo root.

Make sure to not pick the `master` branch on the external repo, so you can open a normal PR to the tool
to merge that subrepo push.

```
git subtree pull -P twitch_oauth2 https://github.com/Emilgardis/twitch_oauth2 master
```

takes all changes since the last `subtree pull` from the external repo
repo and adds these commits to this repo + a merge commit that moves the tool changes into
the specified directory in the twitch_api2 repository.

It is recommended that you always do a push first and get that merged to the tool master branch.
Then, when you do a pull, the merge works without conflicts.
While it's definitely possible to resolve conflicts during a pull, you may have to redo the conflict
resolution if your PR doesn't get merged fast enough and there are new conflicts. Do not try to
rebase the result of a `git subtree pull`, rebasing merge commits is a bad idea in general.

You always need to specify the `-P` prefix to the subtree directory and the corresponding remote
repository. If you specify the wrong directory or repository
you'll get very fun merges that try to push the wrong directory to the wrong remote repository.
Luckily you can just abort this without any consequences by throwing away either the pulled commits
in twitch_api2 or the pushed branch on the remote and try again. It is usually fairly obvious
that this is happening because you suddenly get multiple commits that want to be synchronized.
