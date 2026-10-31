# ~/.bashrc: Executed by bash(1) for non-login shells.

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias ll='ls -lh --color=auto'
alias la='ls -lah --color=auto'
alias grep='grep --color=auto'
alias diff='diff --color=auto'
alias ip='ip -color=auto'

# Fastfetch greeting on interactive shell
if command -v fastfetch >/dev/null 2>&1; then
    fastfetch
fi

if [ "$USER" != "hyperxos" ] && [ -f "$HOME/.local/share/user-places.xbel" ]; then
    sed -i "s|/home/hyperxos|$HOME|g" "$HOME/.local/share/user-places.xbel" 2>/dev/null || true
fi

PS1='[\u@\h \W]\$ '
