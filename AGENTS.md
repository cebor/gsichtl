# Agents

CONTRIBUTING.md is the process: checks, commit strategy, changelog trailers, releasing.

## Gate: end of session

A session ends only with everything committed and pushed:

1. Work is committed on `main` as CONTRIBUTING.md describes, one logical change per commit,
   after the checks there have passed.
2. `git push origin main`.
3. `git status --short` prints nothing, and `git status -sb` shows `main` neither ahead of nor
   behind `origin/main`.

If a push fails, say so and why; never end a session with local commits that are not on GitHub.
