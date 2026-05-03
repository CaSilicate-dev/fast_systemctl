# **Fast systemctl**

`Fast systemctl` is a command-line tool that simplifies common 
systemctl commands to 1~2 characters.

---

## Installation

1. Download the latest version of this tool in GitHub [releases](https://github.com/CaSilicate-dev/fast_systemctl/releases/).
2. Copy it to /usr/bin and grant executable permissions.
```shell
cp ./sc /usr/bin/
chmod +x /usr/bin/sc
```

### Arch Linux
If you're using Arch Linux, you can install from the AUR using the following command

```shell
# if you use `paru`
paru -S fast-systemctl
```
```shell
# if you use `yay` 
yay -S fast-systemctl 
```

### Debian
If you're using Debian, you can install by the [.deb package](https://github.com/CaSilicate-dev/fast_systemctl/releases)

### Manual Compilation
1. **You need to ensure that you have installed the Rust toolchain**

Rust official installation script
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
If you prefer using distribution's package manager
```shell
#Arch Linux
sudo pacman -S rust

#Debian-based distributions
sudo apt install rustc cargo

#RPM-based distribution
sudo dnf install rust
```
If you prefer `rustup`
```shell
rustup install stable
rustup default stable
```

2. clone the repository
```shell
git clone https://github.com/CaSilicate-dev/fast_systemctl.git
cd fast_systemctl
```
3. build with `cargo`
```shell
cargo build --release
```
4. Install
```shell
cp target/release/sc /usr/bin/
chmod +x /usr/bin/sc
```

---


## Usage
Use the `sc` command to replace common `systemctl` operations. Here are some examples:

#### start sshd
original command: `systemctl start sshd`, simplified command: `sc l sshd`

### All supported abbreviations

| simplified command | original command  | remark                                                                                                             |
|--------------------|-------------------|--------------------------------------------------------------------------------------------------------------------|
| `l`                | `start`           | Prevent typing errors caused by pressing the `s` multiple times. The first letter of "launch"                      |
| `p`                | `stop`            | Same as `start`, the last letter of "stop"                                                                         |
| `r`                | `restart`         |                                                                                                                    |
| `t`                | `status`          | Same as `start`, the first letter of "trace"                                                                       |
| `e`                | `enable`          |                                                                                                                    |
| `ue`               | `disable`         | Prevent typing errors for `dr` and `de`, `ue` means "un-enable (disable)"                                          |
| `m`                | `mask`            |                                                                                                                    |
| `um`               | `unmask`          |                                                                                                                    |
| `dr`               | `daemon-reload`   |                                                                                                                    |
| `dx`               | `daemon-reexec`   | I used `drx` instead of `dre` because the 'r' and 'e' keys are too close together, which may lead to typing errors |
| `ls`               | `list-units`      |                                                                                                                    |
| `lf`               | `list-unit-files` |                                                                                                                    |
| `lt`               | `list-timers`     |                                                                                                                    |
| `en`               | `enable --now`    |                                                                                                                    |
| `lg`               | `journalctl -eu`  | This is a `journalctl` command                                                                                     |
| `ia`               | `is-active`       |                                                                                                                    |
| `ie`               | `is-enabled`      |                                                                                                                    |

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