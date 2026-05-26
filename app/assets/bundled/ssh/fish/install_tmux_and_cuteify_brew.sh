brew install tmux

if test $status -eq 0
    tmux -Lcute -CC
    exit
end
