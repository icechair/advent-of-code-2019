#!/bin/bash
target=19690720
targetNoun=19690000
value=0
noun=12
verb=2
while [ $value -lt $target ]; do
    value=`cat input | cargo run --quiet $noun $verb`
    if [ $value -lt "$targetNoun" ]
    then 
        noun=$(($noun+1))
    elif [ $value -lt "$target" ]
    then
        verb=$(($verb+1))
    fi
done
echo $noun $verb $value
