if ($env:DEV -eq "true") {
  cargo build --target $env:TARGET
}
else {
  cargo build --target $env:TARGET --release
}

Remove-Item -Force -Recurse out -ErrorAction SilentlyContinue
New-Item out -ItemType Directory

cbindgen > out/ahqstore_sdk.h

$target = "release"

if ($env:DEV -eq "true") {
  $target = "debug"
}

Get-ChildItem -Path "./target/${env:TARGET}/$target/*" -Include "ahqstore_sdk.dll", "ahqstore_sdk.dylib", "ahqstore_sdk.so", "libahqstore_sdk.dll", "libahqstore_sdk.dylib", "libahqstore_sdk.so" | Copy-Item -Destination "out/"