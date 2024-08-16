# $PATH
export PATH=$HOME/bin:/usr/local/bin:$PATH

# Path to oh-my-zsh installation.
export ZSH="$HOME/.oh-my-zsh"

# Command auto-correction
ENABLE_CORRECTION="true"

# Plugins
plugins=(git you-should-use zsh-autosuggestions zsh-syntax-highlighting)
fpath+=${ZSH_CUSTOM:-${ZSH:-~/.oh-my-zsh}/custom}/plugins/zsh-completions/src
source $ZSH/oh-my-zsh.sh

# Variables
export EDITOR='nvim'
export TERMINAL='tilix'

# Common alias
alias l="eza --icons"
alias ll="eza -a --icons"
alias vim="nvim"
alias ezrc="nvim ~/.zshrc"
alias r="ranger"
alias neo="fastfetch"
# Zypper alias
alias dup="sudo zypper dup"
alias zin="sudo zypper in"
alias zse="zypper se"
alias zrm="sudo zypper rm -u"
alias zrf="sudo zypper refresh"
alias zlr="zypper lr"

# Enable utils
eval "$(zoxide init zsh)"
eval "$(atuin init zsh)"

# Prompt
PROMPT="%{$fg_bold[blue]%}["
PROMPT+="%{$fg_bold[cyan]%}%n%{$fg_bold[blue]%}@%{$fg_bold[cyan]%}%m"
PROMPT+="%{$fg_bold[blue]%}]"
PROMPT+=" %{$fg_bold[cyan]%}%~%{$reset_color%} "
