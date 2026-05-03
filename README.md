# Fast systemctl

`Fast systemctl` is a command-line tool used to simplify systemctl commands.
It simplified some common systemctl commands down to 1~2 characters.

---

## Installation

1. Download the latest version of this tool in GitHub [releases](https://github.com/CaSilicate-dev/fast_systemctl/releases/).
2. copy it to /usr/bin and grant executable permissions.
```shell
cp ./sc /usr/bin/
chmod +x /usr/bin/sc
```

## Usage
Use the `sc` command to replace common `systemctl` operations. Here are some examples:

#### start sshd
original command: `systemctl start sshd`, simplified command: `sc l sshd`

---

### All supported abbreviations

| simplified command | original command  | remark                                                                                                                                  |
|--------------------|-------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `l`                | `start`           | Prevent typing errors caused by pressing the `s` multiple times within a same area of the keyboard, I used the first letter of "launch" |
| `p`                | `stop`            | Same as `start`, the first letter of "parking"                                                                                          |
| `r`                | `restart`         |                                                                                                                                         |
| `t`                | `status`          | Same as `start`, the first letter of "trace"                                                                                            |
| `e`                | `enable`          |                                                                                                                                         |
| `ue`               | `disable`         | Prevent typing errors for `dr` and `de`, `ue` means "un-enable (disable)"                                                               |
| `m`                | `mask`            |                                                                                                                                         |
| `um`               | `unmask`          |                                                                                                                                         |
| `dr`               | `daemon-reload`   |                                                                                                                                         |
| `drx`              | `daemon-reexec`   | I used `drx` instead of `dre` because the 'r' and 'e' keys are too close together, which may lead to typing errors                      |
| `ls`               | `list-units`      |                                                                                                                                         |
| `lf`               | `list-unit-files` |                                                                                                                                         |

## Bash Completion

write the code into .bashrc

```shell
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
```