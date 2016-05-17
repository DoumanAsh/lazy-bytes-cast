#!/bin/bash

function generate_index_redirect() {
    `IFS='/' read -a fields <<< "$TRAVIS_REPO_SLUG"`
    OWNER="${fields[0]}"
    REPO_NAME="${fields[1]}"
    CRATE_NAME=`echo ${REPO_NAME} | sed "s/-/_/g"`

    echo "<!DOCTYPE html>"
    echo "<html lang=\"en-US\">"
    echo "    <head>"
    echo "        <meta charset=\"UTF-8\">"
    echo "        <meta http-equiv=\"refresh\" content=\"1;url=doc/${CRATE_NAME}/index.html\">"
    echo "        <script type=\"text/javascript\">"
    echo "            window.location.href = \"doc/${CRATE_NAME}/index.html\""
    echo "        </script>"
    echo "        <title>Page Redirection</title>"
    echo "    </head>"
    echo "    <body>"
    echo "        <!-- Note: don't tell people to click the link, just tell them that it is a link. -->"
    echo "        If you are not redirected automatically, follow the <a href='doc/${CRATE_NAME}/index.html'>link</a>"
    echo "    </body>"
    echo "</html>"
}

function travis_configure_repo_oauth() {
    git config user.name "Travis CI"
    git config user.email "$COMMIT_AUTHOR_EMAIL"
    echo "https://${GIT_TOKEN}:x-oauth-basic@github.com\n" > ~\.git-credentials
    git config remote.origin.url "https://${GIT_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git"
}

export -f travis_configure_repo_oauth

function upload_pages() {
    echo "Upload github pages"
    if [ -z ${GIT_TOKEN} ]; then
        echo "GIT_TOKEN is not set. Skip github pages update"
        return 0
    fi

    if [ ${UPDATE_PAGES} = "false" ]; then
        echo "Pages update is not needed."
        return 0
    fi

    BRANCH_NAME=$PAGES_BRANCH_NAME

    if [ -z "$1" ] ; then
        BRANCH_NAME=$1
    fi

    # Save some useful information
    REPO=`git config remote.origin.url`
    SHA=`git log -1 --format="%s(%h %cd)" --date=short`

    # Clone the existing gh-pages for this repo into out/
    # Create a new empty branch if gh-pages doesn't exist yet (should only happen on first deply)
    git clone $REPO out
    cd out
    # Creates branch if none exists
    git checkout $BRANCH_NAME || git checkout --orphan $BRANCH_NAME
    cd ..

    # Clean out existing contents
    rm -rf out/doc || return 0

    # Build docs
    cargo doc --no-deps
    cp -rf target/doc/ out/

    cd out
    # If there are no changes to the compiled out (e.g. this is a README update) then just bail.
    if [ -z `git diff --exit-code` ]; then
        echo "No changes to the docs"
        return 0
    fi

    travis_configure_repo_oauth

    git add .
    git commit -m "Auto-update" -m "Commit: ${SHA}"
    git push origin HEAD

    rm -rf out/
}

export -f upload_pages
