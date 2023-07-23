#!/usr/bin/env bash
date= /usr/bin/date
commit="${date}"
# echo "$commit"

git add --all;
git commit -am commit;
git push origin main;