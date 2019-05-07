set -euxo pipefail

main() {
    rustup target add $T
}

main
