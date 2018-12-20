# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
        windows)
            stage=$(mktemp -d)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cargo build --target "$TARGET" --release --verbose --locked

    # copy binary to stage
    cp "target/$TARGET/release/$PROJECT_NAME" $stage/

    # copy completions to stage
    mkdir $stage/complete
    cp target/"$TARGET"/release/build/"$PROJECT_NAME"-*/out/"$PROJECT_NAME".bash $stage/complete/${PROJECT_NAME}.bash-completion
    cp target/"$TARGET"/release/build/"$PROJECT_NAME"-*/out/"$PROJECT_NAME".fish $stage/complete/
    cp target/"$TARGET"/release/build/"$PROJECT_NAME"-*/out/_"$PROJECT_NAME" $stage/complete/
    cp target/"$TARGET"/release/build/"$PROJECT_NAME"-*/out/_"$PROJECT_NAME".ps1 $stage/complete/

    cd $stage
    tar czf $src/$PROJECT_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main