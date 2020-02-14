# JSON-Schema to Nixpkgs module options

Definitely incomplete (and maybe technically too strict) -- works specifically for the config file of matrix-appservice-irc.

Takes JSON (actually YAML, which is theoretically a superset...) as stdin, outputs formatted nix representing a type.
