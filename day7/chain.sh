#!/bin/bash
file="$1"
function run {
printf "$1\n0\n" | ./target/release/day7 $file | xargs -0 \
printf "$2\n%s"  | ./target/release/day7 $file | xargs -0 \
printf "$3\n%s"  | ./target/release/day7 $file | xargs -0 \
printf "$4\n%s"  | ./target/release/day7 $file | xargs -0 \
printf "$5\n%s"  | ./target/release/day7 $file
}
# printf "$1\n$2\n$3\n$4\n$5\n$6\n"
function permutate {
    if [ "${#1}" = 1 ]; then
        perm="${2}${1}"
        run ${perm:0:1} ${perm:1:1} ${perm:2:1} ${perm:3:1} ${perm:4:1}
    else
        for i in $(seq 0 $((${#1}-1)) ); do
            pre="${2}${1:$i:1}"
            seg1="${1:0:$i}"
            seg2="${1:$((i+1))}"
            seg="${seg1}${seg2}"
            permutate "$seg" "$pre"
        done
    fi
}

permutate "01234"
