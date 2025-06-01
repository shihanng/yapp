tf +ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    terraform -chdir=tf {{ ARGS }}

target := `rustc -Vv | awk '/^host:/ { print $2 }'`

test:
    cargo test --target {{ target }}

cover:
    cargo tarpaulin --target {{ target }}

lint:
    pre-commit run --show-diff-on-failure --color=always --all-files

github_token := env('GITHUB_TOKEN', "")

run-ci-local:
    act push {{ if github_token != "" { "-s GITHUB_TOKEN=$GITHUB_TOKEN" } else { "" } }} -j Linters
