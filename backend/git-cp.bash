#!/usr/bin/env bash
date= /usr/bin/date
commit="${date}"
# echo "$commit"

git add --all;
commit | git commit -am -;
git push origin main;