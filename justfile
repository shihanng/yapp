tf +ARGS:
    #!/usr/bin/env bash
    set -euo pipefail
    terraform -chdir=tf {{ ARGS }}
