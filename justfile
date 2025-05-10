tf +ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    terraform -chdir=tf {{ ARGS }}

test target="aarch64-apple-darwin":
    cargo test --target {{ target }}
