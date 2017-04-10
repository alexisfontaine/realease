#!/bin/bash

set -o errexit -o nounset

SOURCE_BRANCH="master"
TARGET_BRANCH="gh-pages"

if [ "$TRAVIS_PULL_REQUEST" != "false" -o "$TRAVIS_BRANCH" != "$SOURCE_BRANCH" ];
then
	echo "This commit was made against the $TRAVIS_BRANCH and not the $SOURCE_BRANCH!"
	exit 0
fi

REPOSITORY=`git config remote.origin.url`
REVISION=$(git rev-parse --short HEAD)

cd public
git init
git remote add upstream ${REPOSITORY/\/\/github.com\//\/\/$GITHUB_OAUTH_TOKEN@github.com\/}
git fetch upstream $TARGET_BRANCH
git reset upstream/$TARGET_BRANCH

git config user.name "Travis CI"
git config user.email "$COMMIT_AUTHOR_EMAIL"

touch .

git add -A .
git commit -m "Build pages at #$REVISION"
git push -q upstream HEAD:$TARGET_BRANCH
