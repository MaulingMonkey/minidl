@pushd "%~dp0.."
cargo build --all || goto :err
cargo test  --all || goto :err
cargo +nightly doc --all --features "nightly" || goto :err
@where wsl >NUL 2>NUL && wsl bash --login -c scripts/test.sh
:err
@popd && exit /b %ERRORLEVEL%
