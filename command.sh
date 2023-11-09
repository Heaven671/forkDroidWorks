#!/bin/bash
command=$1
shift
args=("$@")

cargo_command="cargo run --release $command"

# Ajouter tous les autres arguments directement
for arg in "${args[@]}"; do
    cargo_command+=" $arg"
done

echo "Exécution de la commande : $cargo_command"
eval "$cargo_command"
