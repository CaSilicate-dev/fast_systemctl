_sc_service_completions() {
    local cur prev dirs
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"


    dirs=("/etc/systemd/system/" "/lib/systemd/system/" "/usr/lib/systemd/system/")

    if [[ "$prev" == "dr" || "$prev" == "drx" || "$prev" == "ls" || "$prev" == "lf" ]]; then
        COMPREPLY=()
        return 0
    else
        for dir in "${dirs[@]}"; do

            COMPREPLY+=( $(compgen -f "$dir" | sed "s|$dir||" | grep -i "^$cur") )
        done

        if [ ${#COMPREPLY[@]} -eq 1 ]; then
            COMPREPLY=( "${COMPREPLY[0]}" )
        fi
        return 0

    fi
}

complete -F _sc_service_completions sc
