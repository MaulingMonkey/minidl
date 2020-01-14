function print_run {
    printf "\033[1;32m$(whoami)@$(hostname)\033[0m:\033[1;34m$(pwd)\033[0m$ "
    echo "$@"
    "$@"
}

print_run cargo build --all || exit 1
print_run cargo test  --all || exit 1
