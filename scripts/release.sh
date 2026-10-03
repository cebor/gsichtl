#!/usr/bin/env bash
# Release gsichtl: a changelog section, the version bump, an annotated tag.
#
#     scripts/release.sh 0.3.0
#
# The changelog section is generated from the `Changelog:` trailers since the last tag, unless
# CHANGELOG.md already has a section for this version — then that one is used as written. So
# to edit the wording before releasing, generate it once, edit and commit it, and run again.
#
# Nothing is pushed. The last line printed is the push command.

set -euo pipefail

die() { echo "release: $*" >&2; exit 1; }

# The sed expressions below are GNU's (`-i` without a suffix). On macOS that is usually
# installed as gsed.
if sed --version >/dev/null 2>&1; then
    sed=sed
elif command -v gsed >/dev/null; then
    sed=gsed
else
    die "needs GNU sed; on macOS: brew install gnu-sed"
fi

cd "$(git rev-parse --show-toplevel)"

[[ $# -eq 1 ]] || die "usage: scripts/release.sh <major.minor.patch>"
version=$1
tag="v$version"
[[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "'$version' is not major.minor.patch"

[[ $(git symbolic-ref --short HEAD 2>/dev/null) == main ]] || die "not on main"
[[ -z $(git status --porcelain) ]] || die "the working tree is not clean"
! git rev-parse -q --verify "refs/tags/$tag" >/dev/null || die "$tag already exists"

last_tag=$(git tag --list 'v[0-9]*' --sort=-v:refname | head -n1)
if [[ -n $last_tag ]]; then
    [[ $(git rev-parse HEAD) != $(git rev-parse "$last_tag^{commit}") ]] ||
        die "nothing to release: HEAD is $last_tag"
    newest=$(printf '%s\n' "${last_tag#v}" "$version" | sort -V | tail -n1)
    [[ $newest == "$version" && ${last_tag#v} != "$version" ]] ||
        die "$version is not above the last release, $last_tag"
fi

# The two places the version is written down. They must agree before the release and after it.
manifest_version() { $sed -n '/^\[package\]/,/^\[/ s/^version = "\(.*\)"/\1/p' Cargo.toml; }
lock_version() { awk '/^name = "gsichtl"$/ { getline; gsub(/"/, "", $3); print $3 }' Cargo.lock; }

current=$(manifest_version)
locked=$(lock_version)
[[ $current == "$locked" ]] ||
    die "Cargo.toml is at $current but Cargo.lock at $locked; fix that by hand first"

# --- The changelog section ----------------------------------------------------------------

# Prints the body of CHANGELOG.md's section for $1, without its heading.
section_of() {
    awk -v head="## [$1]" '
        index($0, head) == 1 { inside = 1; next }
        inside && /^## \[/ { exit }
        inside { print }
    ' CHANGELOG.md
}

if grep -qF "## [$version]" CHANGELOG.md; then
    echo "Using the section for $version already in CHANGELOG.md."
else
    entries=""
    range=${last_tag:+$last_tag..}HEAD
    while IFS=$'\x1f' read -r -d $'\x1e' hash subject kinds; do
        hash=${hash//$'\n'/}
        kinds=${kinds//$'\n'/}
        [[ -n $kinds ]] || continue
        [[ $kinds != *,* ]] || die "$hash has more than one Changelog trailer: $kinds"
        case $kinds in
            added | changed | deprecated | removed | fixed | security | performance) ;;
            *) die "$hash has an unknown Changelog trailer '$kinds' (see CONTRIBUTING.md)" ;;
        esac
        entries+="$kinds - $subject ($hash)"$'\n'
    done < <(git log --reverse --format='%h%x1f%s%x1f%(trailers:key=Changelog,valueonly,separator=%x2C)%x1e' "$range")

    section="## [$version] - $(date +%F)"$'\n'
    for kind in Added Changed Deprecated Removed Fixed Security Performance; do
        lower=$(tr '[:upper:]' '[:lower:]' <<<"$kind")
        lines=$(awk -v k="$lower" '$1 == k { sub(/^[^ ]+ /, ""); print }' <<<"$entries")
        [[ -n $lines ]] || continue
        section+=$'\n'"### $kind"$'\n\n'"$lines"$'\n'
    done
    [[ $section == *'### '* ]] || die "nothing user-visible since ${last_tag:-the first commit}"

    # Newest first: in front of the first existing section, or at the end if there is none.
    section_file=$(mktemp)
    trap 'rm -f "$section_file"' EXIT
    printf '%s\n' "$section" >"$section_file"
    awk -v file="$section_file" '
        !done && /^## \[/ { while ((getline line < file) > 0) print line; done = 1 }
        { print }
        END { if (!done) { print ""; while ((getline line < file) > 0) print line } }
    ' CHANGELOG.md >CHANGELOG.md.new
    mv CHANGELOG.md.new CHANGELOG.md
    echo "Wrote the section for $version into CHANGELOG.md."
fi

# --- The version bump ---------------------------------------------------------------------

if [[ $version != "$current" ]]; then
    trap 'echo "release: failed while bumping; git restore . puts everything back" >&2' ERR
    $sed -i "/^\[package\]/,/^\[/ s/^version = \".*\"/version = \"$version\"/" Cargo.toml
    cargo update --workspace --quiet
    [[ $(manifest_version) == "$version" && $(lock_version) == "$version" ]] ||
        die "the bump missed Cargo.toml or Cargo.lock"
fi

if [[ -n $(git status --porcelain) ]]; then
    git add -A
    git commit --quiet -m "Release $version"
    echo "Committed Release $version."
fi

# --cleanup=verbatim, because the default would strip the ### headings as comments.
git tag -a "$tag" --cleanup=verbatim -F - <<EOF
gsichtl $version
$(section_of "$version")
EOF
echo "Tagged $tag."
echo
echo "    git push origin main $tag"
