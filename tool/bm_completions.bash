#!/usr/bin/env bash

# Tab complete tags
# Example: https://www.baeldung.com/linux/shell-auto-completion#3-sample-function
function _tab_complete_bm()
{
  latest="${COMP_WORDS[$COMP_CWORD]}"
  prev="${COMP_WORDS[$COMP_CWORD - 1]}"
  words=""
  case "${prev}" in
    --tag | -t)
      words=$(bm tags --machine)
      ;;
    *)
      ;;
  esac
  # shellcheck disable=SC2207
  COMPREPLY=( $(compgen -W "$words" -- "$latest") )
  return 0
}

# Add completions using fzf if it is installed
if type fzf &>/dev/null && type __fzf_defc &>/dev/null; then
  # Allow `bm s -t **<tab>` to complete the tag with fzf
  # Reference: https://thevaluable.dev/fzf-shell-integration/
  _fzf_complete_bm() {
    cur="${COMP_WORDS[$COMP_CWORD]}"
    prev="${COMP_WORDS[$COMP_CWORD - 1]}"
    trigger=${FZF_COMPLETION_TRIGGER-'**'}
    if [[ $prev == '--tag' ]] || [[ $prev == "-t" ]]; then
      # From `fzf --bash`, check if the fzf trigger is being used and then complete with fzf
      # Otherwise, fall back to normal tab completion
      # shellcheck disable=SC2016
      if [[ "$cur" == *"$trigger" ]] && [[ $cur != *'$('* ]] && [[ $cur != *':='* ]] && [[ $cur != *'`'* ]]; then
        _fzf_complete -- "$@" < <(
          bm tags --machine
        )
      else
        _tab_complete_bm "$@"
      fi
    fi
  }

  # Tell fzf to run for the bm command
  # Discovered this was necessary by running `fzf --bash|less` and reading how it works towards the bottom
  __fzf_defc "bm" _fzf_complete_bm "-o default -o bashdefault"
else
  # Only give installation errors during interactive shells so it doesn't mess up things like ssh
  #   https://serverfault.com/a/146747
  if type fzf &>/dev/null && [[ $- == *i* ]]; then
      echo "ERROR CONFIGURING BM: It looks like fzf is installed but it wasn't initialized before bm so bm/fzf integration not setup.  Initialize fzf before BM in your bashrc."
  fi
  # If fzf isn't installed, still do regular tab completion
  complete -F _tab_complete_bm bm
fi
