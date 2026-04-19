[group('flake')]
check:
    nix flake check

[group('flake')]
update *input:
    nix flake update {{input}}

[group('develop')]
dev shell=("unset"):
    #!/usr/bin/env sh
    if [ {{shell}} = "unset" ]; then
        nix develop -c "$(basename $SHELL)"
    else
        nix develop -c {{shell}}
    fi

[group('package')]
run:
    nix run .
