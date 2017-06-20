cargo doc --no-deps
if (Test-Path -Path docs) { Remove-Item -Recurse -Path docs }
Copy-Item -Recurse -Path target/doc -Destination docs
Write-Output '<meta http-equiv=refresh content=0;url=hipchat_client/index.html>' > docs/index.html
