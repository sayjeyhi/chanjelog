#!/bin/bash

# shellcheck disable=SC2154
# shellcheck disable=SC2034
# shellcheck disable=SC2046
# shellcheck disable=SC2099
# shellcheck disable=SC2028
# shellcheck disable=SC2116
# shellcheck disable=SC2129

NOW=$(date +'%Y-%m-%d %H:%M:%S')
CURRENT_GIT_VERSION=$(git describe --tags --abbrev=0 || '1.0.0')
NEW_GIT_VERSION=$(bash ./utils/semver bump patch $CURRENT_GIT_VERSION)
NEW_TAG_CHANGES=$(git log --no-merges --pretty=format:"- %s (**%cn** - %cr)" HEAD...$CURRENT_GIT_VERSION)

echo "## $NEW_GIT_VERSION" > changeslog.md
echo $NOW >> changeslog.md
echo "" >> changeslog.md
echo "$NEW_TAG_CHANGES" >> changeslog.md
echo "" >> changeslog.md
echo "" >> changeslog.md
cat ../changeslog.md >> changeslog.md
mv changeslog.md ../changeslog.md
git tag -a $NEW_GIT_VERSION -m "$NEW_TAG_CHANGES"
