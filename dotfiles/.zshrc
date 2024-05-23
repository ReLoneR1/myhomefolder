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

# Editor
 if [[ -n $SSH_CONNECTION ]]; then
   export EDITOR='vim'
 else
   export EDITOR='nvim'
 fi

# Common alias
alias l="eza --icons"
alias ll="eza -a --icons"
alias vim="nvim"
alias ezrc="nvim ~/.zshrc"
alias r="ranger"
alias wow="nvim ~/Documents/wow/wow.norg"
# xbps alias
alias xsu="sudo xbps-install -Su"
alias xin="sudo xbps-install -S"
alias xqu="xbps-query -Rs"
alias xre="sudo xbps-remove -R"
# xbps-src alias
alias xsus='./xbps-src update-sys'
alias xins='./xbps-src pkg'
# xdeb alias
alias xind='./xdeb -Sedf'

# Enable utils
eval "$(zoxide init zsh)"
eval "$(atuin init zsh)"

# Prompt
PROMPT="%{$fg_bold[magenta]%}["
PROMPT+="%{$fg_bold[green]%}%n%{$fg_bold[blue]%}@%{$fg_bold[magenta]%}%m"
PROMPT+="%{$fg_bold[magenta]%}]"
PROMPT+=" %{$fg_bold[green]%}%~%{$reset_color%} "
