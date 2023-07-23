$commit = Get-Date
Write-Output $commit

git add --all
git commit -am (Get-Date -Format "ddd-MMM-dd_HH:mm:ss")
git push origin main
