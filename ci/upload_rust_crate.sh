#!/bin/bash

function cargo_publish() {
    export UPDATE_PAGES="true"
    cargo login $CARGO_API
    cargo publish
}

function upload_rust_crate() {
    echo "Upload Rust crate"
    # Set CARGO_API to update crate and its docs
    if [ -z ${CARGO_API} ]; then
        echo "CARGO_API is not set. Skip update of crate"
        return 0
    fi

    echo "Check if crate publish is required"
    cd $TRAVIS_BUILD_DIR
    # Check crate version
    CRATE_VER=`cargo search lazy-bytes-cast | grep -oP "[0-9]\.[0-9]\.[0-9]"`
    CURRENT_VER=`grep -oP "[0-9]\.[0-9]\.[0-9]" Cargo.toml`
    echo "Crate current version=${CRATE_VER}"
    echo "Repo current version=${CURRENT_VER}"
    # Check if crate update is needed
    if [ -z "$CRATE_VER" ]; then
        echo "Not uploaded yet"

        if [ "$CARGO_UPLOAD_FIRST_TIME" == "true" ]; then
            echo "Upload first time"
            cargo_publish
        fi
    else
        function version_gt() { test "$(echo "$@" | tr " " "\n" | sort -V | head -n 1)" != "$1"; }

        if version_gt $CURRENT_VER $CRATE_VER; then
            cargo_publish
        fi
    fi

    cd -
}

upload_rust_crate
