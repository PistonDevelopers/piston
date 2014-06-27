#!/bin/bash
# written by bvssvni
# Modify the setting to do conditional compilation.
# For example "--cfg my_feature"
SETTINGS=""
# ================================================

MAKE=make
if [ "$OS" == "Windows_NT" ]; then
    MAKE=mingw32-make
fi

# Checks if an item exists in an array.
# Copied from http://stackoverflow.com/questions/3685970/check-if-an-array-contains-a-value
function contains() {
    local n=$#
    local value=${!n}
    for ((i=1;i < $#;i++)) {
        if [ "${!i}" == "${value}" ]; then
            echo "y"
            return 0
        fi
    }
    echo "n"
    return 1
}

# This is a counter used to insert dependencies.
# It is global because we need an array of all the
# visited dependencies.
i=0
function build_deps {
    local current=$(pwd)
    for symlib in $(find target/deps/ -type l) ; do
        cd $current
        echo $symlib
        local original_file=$(readlink $symlib)
        local original_dir=$(dirname $original_file)
        cd $original_dir

        # Go to the git root directory.
        local current_git_dir=$(git rev-parse --show-toplevel)
        echo "--- Parent $current"
        echo "--- Child $current_git_dir"
        cd $current_git_dir

        # Skip building if it is already built.
        if [ $(contains "${git_dir[@]}" $current_git_dir) == "y" ]; then
            echo "--- Visited $current_git_dir"
            continue
        fi

        # Remember git directory to not build it twice
        git_dir[i]=$current_git_dir

        # Visit the symlinks and build the dependencies
        build_deps

        # First check for a 'build.sh' script with default settings.
        # Check for additional 'rust-empty.mk' file. # Compile with the settings flags. # If no other options, build with make.
        ( test -e build.sh && ./build.sh ) || ( test -e rust-empty.mk && $MAKE -f rust-empty.mk clean && $MAKE -f rust-empty.mk ) || ( echo "--- Building $current_git_dir" && $MAKE clean && $MAKE )
        let i+=1
    done
    cd $current
}

# Mark main project as visited to avoid infinite loop.
git_dir[i]=$(pwd)
let i+=1
if [ "$1" == "deps" ]; then
    build_deps
fi

echo "--- Building $(pwd)"
( test -e rust-empty.mk && $MAKE -f rust-empty.mk clean && $MAKE -f rust-empty.mk COMPILER_FLAGS+="$SETTINGS" ) || ( $MAKE clean
    $MAKE COMPILER_FLAGS+="$SETTINGS"
)

