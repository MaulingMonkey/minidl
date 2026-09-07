@pushd "%~dp0.."
cargo +nightly doc --all --all-features %*
:err
@popd && exit /b %ERRORLEVEL%
