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
alias wow="nvim ~/Документы/wow/wow.norg"
alias neo="fastfetch"
# zypper alias
alias dup="sudo zypper dup"
alias zin="sudo zypper in"
alias zse="zypper se"
alias zrm="sudo zypper rm -u"

# Enable utils
eval "$(zoxide init zsh)"
eval "$(atuin init zsh)"

# Prompt
PROMPT="%{$fg_bold[magenta]%}["
PROMPT+="%{$fg_bold[green]%}%n%{$fg_bold[blue]%}@%{$fg_bold[magenta]%}%m"
PROMPT+="%{$fg_bold[magenta]%}]"
PROMPT+=" %{$fg_bold[green]%}%~%{$reset_color%} "
