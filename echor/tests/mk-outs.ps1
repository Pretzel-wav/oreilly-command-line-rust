$scriptPath = $MyInvocation.MyCommand.Path
$scriptFolder = Split-Path $scriptPath -Parent

$out_dir = "$scriptFolder\expected"

if (-Not (Test-Path $out_dir)) {
    New-Item -ItemType Directory -Path $out_dir
}

Write-Output "Hello there" > "$out_dir\hello1.txt"
Write-Output "Hello"    "there" > "$out_dir\hello2.txt"
Write-Output -n "Hello    there" > "$out_dir\hello1.n.txt"
Write-Output -n "Hello" "there" > "$out_dir\hello2.n.txt"

Write-Output "Created test files in $out_dir"