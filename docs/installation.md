# RoyaOS Installation Guide

This comprehensive guide provides detailed instructions for installing, configuring, and running RoyaOS, the operating system designed specifically for Roya AGI.

## System Requirements

### Minimum Requirements
- **Operating System**: Linux (Ubuntu 20.04 or newer recommended), Windows 10/11, or macOS 12+
- **CPU**: 4+ cores (x86_64 architecture)
- **RAM**: 8GB minimum
- **Storage**: 20GB+ of free space
- **Rust**: 1.70.0 or newer

### Recommended Requirements
- **CPU**: 8+ cores
- **RAM**: 16GB or more
- **Storage**: 100GB+ SSD
- **GPU**: CUDA-compatible GPU with 8GB+ VRAM (for accelerated AGI operations)
- **Network**: High-speed internet connection

## Pre-Installation Checklist

Before beginning the installation, ensure you have:

- [ ] Administrator/root access to your system
- [ ] Updated your operating system to the latest version
- [ ] Backed up any important data
- [ ] Disabled any security software that might interfere with the installation
- [ ] Stable internet connection for downloading dependencies

## Installation Steps

### 1. Install Dependencies

#### For Ubuntu/Debian-based systems:
```bash
sudo apt update
sudo apt install -y git build-essential pkg-config libssl-dev cmake curl
```

#### For Fedora/RHEL-based systems:
```bash
sudo dnf install -y git gcc gcc-c++ make pkgconfig openssl-devel cmake curl
```

#### For macOS (using Homebrew):
```bash
brew install openssl cmake pkg-config
```

#### For Windows:
Install the following:
- [Git for Windows](https://gitforwindows.org/)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [CMake](https://cmake.org/download/)

### 2. Install Rust

If you don't have Rust installed, install it using rustup:

#### For Linux/macOS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### For Windows:
```bash
curl -sSf https://sh.rustup.rs | sh
```

Verify the installation:
```bash
rustc --version
cargo --version
```

### 3. Clone the Repository

```bash
git clone https://github.com/your-username/royaos.git
cd royaos
```

### 4. Run the Installation Script

The installation script will set up everything you need:

```bash
chmod +x install.sh
./install.sh
```

The script performs the following actions:
- Verifies system requirements
- Installs required dependencies
- Sets up the development environment
- Creates necessary directories
- Configures initial settings

### 5. Configure RoyaOS

After installation, you'll need to configure RoyaOS to match your environment and requirements:

1. Copy the example configuration file:
   ```bash
   cp config/config.example.yaml config/config.yaml
   ```

2. Edit the configuration file to match your requirements:
   ```bash
   nano config/config.yaml
   ```

   Key configuration sections include:
   - **System**: Basic system settings
   - **Memory**: Memory allocation and optimization
   - **Tools**: Tool discovery and integration
   - **Security**: Security policies and permissions

3. Configure environment-specific settings:
   - For GPU acceleration (if available):
     ```bash
     echo "CUDA_VISIBLE_DEVICES=0,1" >> .env
     ```
   - For custom data directories:
     ```bash
     echo "ROYAOS_DATA_DIR=/path/to/data" >> .env
     ```

### 6. Build RoyaOS

Build the project using Cargo:

```bash
cargo build --release
```

This process may take several minutes depending on your system. The `--release` flag ensures optimized performance.

### 7. Run RoyaOS

Start RoyaOS:

```bash
cargo run --release
```

You should see output indicating that RoyaOS is starting up. The system will initialize its components and be ready to connect with Roya AGI.

## Post-Installation Steps

### 1. Verify Installation

Verify that RoyaOS is running correctly:

```bash
curl http://localhost:8000/status
```

You should receive a JSON response with system status information.

### 2. Set Up Roya AGI Integration

To connect Roya AGI with RoyaOS:

1. Install the Roya AGI connector:
   ```bash
   cargo install roya-connector
   ```

2. Configure the connector:
   ```bash
   roya-connector --config path/to/connector/config.yaml
   ```

3. Test the connection:
   ```bash
   roya-connector --test
   ```

### 3. Set Up Automatic Startup (Optional)

To configure RoyaOS to start automatically on system boot:

#### For systemd-based Linux systems:
1. Create a service file:
   ```bash
   sudo nano /etc/systemd/system/royaos.service
   ```

2. Add the following content:
   ```
   [Unit]
   Description=RoyaOS - Operating System for Roya AGI
   After=network.target

   [Service]
   Type=simple
   User=<your-username>
   WorkingDirectory=/path/to/royaos
   ExecStart=/path/to/cargo run --release
   Restart=on-failure

   [Install]
   WantedBy=multi-user.target
   ```

3. Enable and start the service:
   ```bash
   sudo systemctl enable royaos
   sudo systemctl start royaos
   ```

## Troubleshooting

If you encounter issues during installation:

### Common Issues and Solutions

#### Build Failures
- **Issue**: Cargo build fails with dependency errors
- **Solution**: Update Rust and try again: `rustup update`

#### Permission Errors
- **Issue**: Permission denied when running RoyaOS
- **Solution**: Check file permissions and ensure you have the necessary access rights

#### Configuration Errors
- **Issue**: RoyaOS fails to start due to configuration issues
- **Solution**: Verify your `config.yaml` file for syntax errors

#### Memory Allocation Failures
- **Issue**: System reports memory allocation errors
- **Solution**: Adjust the `max_allocation` setting in your configuration file

### Diagnostic Steps

1. Check the logs in the `logs` directory for detailed error information
2. Ensure all dependencies are correctly installed
3. Verify your Rust version with `rustc --version`
4. Run with debug output: `RUST_LOG=debug cargo run`

### Getting Help

If you continue to experience issues:
- Check the [GitHub Issues](https://github.com/your-username/royaos/issues) for known problems
- Join the [RoyaOS Community Forum](https://community.royaos.org) for community support
- Contact the development team at support@royaos.org

## Advanced Configuration

For advanced users who need to customize RoyaOS further:

### Custom Tool Integration

To integrate custom tools with RoyaOS:

1. Create a tool manifest file in the `tools` directory
2. Implement the tool interface as described in the [Tool Development Guide](tool_development.md)
3. Register the tool with RoyaOS using the Tool Manager API

### Security Customization

To customize security policies:

1. Edit the security section in `config.yaml`
2. Create custom permission sets in `security/permissions.yaml`
3. Implement custom security checks if needed

### Performance Tuning

For optimal performance:

1. Adjust memory allocation based on your system capabilities
2. Configure the optimization strategy in the memory section
3. Enable multi-threading for compute-intensive operations

## Next Steps

After installation, refer to these resources for further information:

- [User Guide](user_guide.md) - Comprehensive guide to using RoyaOS with Roya AGI
- [API Documentation](api_docs.md) - Details on the RoyaOS API for developers
- [Architecture Overview](architecture.md) - In-depth explanation of RoyaOS architecture
- [Contributing Guide](contributing.md) - How to contribute to the RoyaOS project

## Upgrading RoyaOS

To upgrade to a newer version of RoyaOS:

1. Pull the latest changes:
   ```bash
   git pull origin main
   ```

2. Run the upgrade script:
   ```bash
   ./scripts/upgrade.sh
   ```

3. Rebuild the project:
   ```bash
   cargo build --release
   ```

4. Restart RoyaOS:
   ```bash
   cargo run --release
   ```
