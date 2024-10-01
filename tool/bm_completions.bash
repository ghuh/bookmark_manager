#!/usr/bin/env bash

# Add completions using fzf if it is installed
if type fzf &>/dev/null; then
  # Allow `bm s -t **<tab>` to complete the tag with fzf
  # Reference: https://thevaluable.dev/fzf-shell-integration/
  _fzf_complete_bm() {
    # Only complete for the tag option
    # https://stackoverflow.com/a/1854031
    LAST="${*: -1}"
    if [[ $LAST == '--tag' ]] || [[ $LAST == "-t" ]]; then
      _fzf_complete -- "$@" < <(
        bm tags --machine
      )
    fi
  }

  # Tell fzf to run for the bm command
  # Discovered this was necessary by running `fzf --bash|less` and reading how it works towards the bottom
  __fzf_defc "bm" _fzf_complete_bm "-o default -o bashdefault"
fi
