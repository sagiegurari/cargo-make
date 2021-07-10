#/usr/bin/env bash

_cargo_make_completions()
{
    if [ "${#COMP_WORDS[@]}" != "2" ]; then
        return
    fi

    # add cli options
    ALL_WORDS="--allow-private --diff-steps --disable-check-for-updates --experimental -h --help --list-all-steps --no-color --no-on-error --no-workspace --print-steps -skip-init-end-tasks --time-summary -v --version -V --version --cwd -e --env --env-file -l --loglevel verbose info error --makefile --output-format default short-description markdown markdown-single-page markdown-sub-section autocomplete --output-file -p --profile --skip-tasks -t --task "

    # add task names
    ALL_WORDS=("${ALL_WORDS}$(makers --loglevel error --list-all-steps --output-format autocomplete)")

    COMPREPLY=($(compgen -W "$ALL_WORDS" -- "${COMP_WORDS[1]}"))
}

complete -F _cargo_make_completions makers

