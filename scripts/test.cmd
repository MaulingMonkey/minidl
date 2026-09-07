@pushd "%~dp0.."
cargo build --all || goto :err
cargo test  --all || goto :err
cargo test  --all --all-features || goto :err
cargo test  --all                       --features "winresult" || goto :err
cargo test  --all --no-default-features --features "alloc" || goto :err
cargo test  --all --no-default-features --features ""      || goto :err
cargo +stable build --all || goto :err
cargo +stable test  --all || goto :err
@call scripts\doc || goto :err
@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh
:err
@popd && exit /b %ERRORLEVEL%
