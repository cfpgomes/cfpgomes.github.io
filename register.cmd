cargo build
copy appx\* target\debug
cd target\debug
powershell -command "Get-AppxPackage *personal-website-manager* | Remove-AppxPackage"
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
cd ..\