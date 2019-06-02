set -euxo pipefail

main() {
    cargo check -p ufmt --target $T

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check -p ufmt-utils --target $T

        case $T in
            *-unknown-linux-*)
                cargo test --target $T --features std
                ;;

            thumbv7m-none-eabi)
                cd nopanic

                cargo build --examples --release
                size $(find target/thumbv7m-none-eabi/release/examples \
                            -executable \
                            -type f \
                            ! -name '*-*' | sort)
                ;;
        esac

        if [ $T = x86_64-unknown-linux-gnu ]; then
            ( cd macros && cargo test )
        fi
    fi
}

# fake Travis variables to be able to run this on a local machine
if [ -z ${TRAVIS_BRANCH-} ]; then
    TRAVIS_BRANCH=auto
fi

if [ -z ${TRAVIS_PULL_REQUEST-} ]; then
    TRAVIS_PULL_REQUEST=false
fi

if [ -z ${TRAVIS_RUST_VERSION-} ]; then
    case $(rustc -V) in
        *nightly*)
            TRAVIS_RUST_VERSION=nightly
            ;;
        *beta*)
            TRAVIS_RUST_VERSION=beta
            ;;
        *)
            TRAVIS_RUST_VERSION=stable
            ;;
    esac
fi

if [ -z ${T-} ]; then
    T=$(rustc -Vv | grep host | cut -d ' ' -f2)
fi

if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_PULL_REQUEST != false ]; then
    main
fi
