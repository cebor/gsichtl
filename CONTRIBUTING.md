# Contributing to gsichtl

gsichtl is a small crate with one promise: the same seed draws the same picture. Most of what
follows exists to keep that promise, and to keep the history readable enough that the reason
behind every picture can still be found.

## Setting up

You need stable Rust; the crate's minimum is Rust 1.91, edition 2024. CI runs these, and a
change should pass them before it is pushed:

```bash
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cargo test --all-features --locked
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked
```

Both feature sets matter: `Avatar::to_png` and its imports exist only with the `png` feature.

`scripts/release.sh` also needs GNU sed. On macOS, `brew install gnu-sed` installs it as
`gsed`, which the script finds.

## Before you change something

- **Pictures are a contract.** As README.md's "Stability" says, changing a part, a palette,
  the draw order or a context string changes the pictures seeds produce, and needs a minor
  version bump. `seeds_keep_their_pictures` in `tests/gsichtl.rs` fails until its fingerprints
  are updated; update them only when the change is meant to move pictures.
- **Context strings are documented.** README.md's "How it works" lists every generator's
  context string; a new generator or a new context string updates that list in the same commit.
- **Sample sheets follow the pictures.** A change that moves what `docs/faces.png` or
  `docs/groups.png` show regenerates them, with the commands in README.md's "Sample sheets", in
  the same commit.
- **No unsafe code.** The crate sets `unsafe_code = "forbid"`.
- **New dependencies: ask first.** Open an issue before adding one.

## Commit strategy

The history of `main` is linear and meant to be read. `git log` is where the reasoning behind
the code lives, so a commit is written for the person who runs `git blame` on it a year from
now.

### Branches and merging

- Work on a short-lived topic branch off `main`, one branch per change, and open a pull
  request from it.
- Keep it current by **rebasing** onto `main`, never by merging `main` into it.
- It lands **fast-forward only**. There are no merge commits on `main`, and no squash-on-merge
  either — the commits you push are the commits that land, so shape them before review
  finishes.
- Never force-push `main`. Force-pushing your own topic branch after a rebase is expected
  (`git push --force-with-lease`).

### What goes into one commit

- **One logical change per commit.** A feature, a fix, a refactor — not two of them, and not
  half of one. If the message needs "and also", it is two commits.
- **Every commit is green on its own.** The checks above pass at every commit, not just at the
  tip, so `git bisect` always lands somewhere meaningful.
- **Docs and sample sheets change with their cause.** If a change makes a sentence in the
  README untrue, or moves a picture on a sample sheet, fix the sentence or regenerate the sheet
  in the same commit.
- Review fixes are folded into the commit they fix (`git commit --fixup`, then
  `git rebase -i --autosquash main`) rather than piled on top as "address review".

### Writing the message

```
Add badges: round emblems for groups

Crests are shields, and a group that is not a guild looks odd behind
one. A badge is a 12x12 disc with a division, an emblem and sometimes
a rim, drawn from its own context string, so no existing seed changes
its face, monster, nerd or crest.

`Kind` gains a `Badge` variant; a `match` on `Kind` without a wildcard
arm needs one more arm.

Changelog: added
```

- **Subject:** imperative, says what the change does, no trailing full stop, at most about 60
  characters. No `feat:`/`fix:` prefixes — the changelog comes from a trailer instead (below),
  so the subject can stay a sentence.
- **Body:** wrapped at 72 columns, and says **why**: what was wrong, what was tried, and why
  this fix over the alternatives.
- Written in English, like the rest of the repository.

### The changelog trailer

`CHANGELOG.md` is generated from a `Changelog:` trailer in the last paragraph of the message.
Add it when **someone using the crate would notice the change** — in the public API, in a
picture a seed produces, in the `png` feature, or in the minimum Rust version. Leave it off for
refactors, tests, CI and documentation; those commits do not appear in the changelog at all.

| Value | For |
|---|---|
| `added` | a new capability: a generator, a function, a feature |
| `changed` | existing behaviour that works differently, including a picture a seed produces |
| `fixed` | a bug that users could hit |
| `performance` | the same pictures, measurably faster or lighter — say by how much in the body |
| `deprecated` | still works, will be removed; the body names the replacement |
| `removed` | gone |
| `security` | a vulnerability fix |

One value per commit. If a commit seems to need two, it is usually two commits. The subject
line becomes the changelog entry, which is one more reason to write it as a sentence a user
can understand.

A changed picture or a breaking change to the public API is `changed` or `removed`, and its
body says what users have to do. `git commit --trailer "Changelog: fixed"` adds the line in
the right place.

The history before this convention has no trailers, so everything up to and including 0.2.0 is
summarised by hand.

## Releasing

`scripts/release.sh` is the only thing that changes the version:

```bash
scripts/release.sh 0.3.0
git push origin main v0.3.0
```

On a clean `main` it collects the `Changelog:` trailers since the last tag into a new
`CHANGELOG.md` section. It then raises the version in `Cargo.toml` and updates `Cargo.lock` to
match, commits that as `Release 0.3.0` and creates the annotated tag `v0.3.0`, with the section
as the tag message. It never pushes.

It stops before changing anything if a trailer has an unknown value or a commit has more than
one, if nothing user-visible happened since the last tag, or if the version is not above the
last one. It also stops if `Cargo.toml` and `Cargo.lock` already disagree about the version.

To reword the generated entries, let the script write the section, commit the edited
`CHANGELOG.md`, delete the tag and the `Release` commit it made
(`git tag -d v0.3.0 && git reset --hard HEAD~1`), and run it again. A section that already
exists for the version is used as written.

Below 1.0, a changed picture or a breaking API change raises the minor version; everything
else raises the patch version (README.md, "Stability").

Pushing the tag runs `.github/workflows/release.yml`: the CI suites again, a check that the tag
matches `Cargo.toml` and that `CHANGELOG.md` has a section for it, then `cargo publish` to
crates.io, and finally the GitHub release `gsichtl 0.3.0` with that section as its notes. The
publish job runs in the `release` environment and authenticates by trusted publishing, so no
token is stored anywhere. A version already on crates.io and a GitHub release that already
exists are skipped, so a run that failed halfway can be re-run as it is.

crates.io only accepts a trusted publisher for a crate that already exists, so 0.1.0 was
published by hand; every release since goes through the workflow.

## License

gsichtl is MIT-licensed. By contributing you agree that your contribution is released under
the same [license](LICENSE).
