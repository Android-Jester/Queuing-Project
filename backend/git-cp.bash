#!/usr/bin/env bash
commit=$(date)
echo $commit

git add --all;
git commit -am `date '+%a-%b-%d_%H:%M:%S'`;
git push origin main;