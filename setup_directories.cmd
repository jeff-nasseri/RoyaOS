@echo off
REM Script to set up the RoyaOS directory structure

echo Creating RoyaOS directory structure...

REM Create main directories
mkdir crates\kernel\src
mkdir crates\memory\src
mkdir crates\tools\src
mkdir crates\security\src
mkdir crates\interface\src
mkdir config
mkdir docs
mkdir features
mkdir src
mkdir logs
mkdir data\memory
mkdir data\storage

echo Directory structure created successfully.
echo.
echo Next steps:
echo 1. Run 'cargo build' to compile the project
echo 2. Copy config\config.example.yaml to config\config.yaml
echo 3. Edit config\config.yaml to match your requirements
echo 4. Run 'cargo run' to start RoyaOS
