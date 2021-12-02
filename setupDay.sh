cp -r template day$1
cd day$1
curl --cookie "session=$AOCSESSION" https://adventofcode.com/2021/day/$1/input > input
