# RoyaOS User Guide

This comprehensive guide provides detailed instructions for using RoyaOS with Roya AGI. It covers basic operations, advanced features, and best practices for getting the most out of your AGI operating system.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Basic Operations](#basic-operations)
4. [Memory Management](#memory-management)
5. [Tool Integration](#tool-integration)
6. [Security Management](#security-management)
7. [Interface and Communication](#interface-and-communication)
8. [Advanced Features](#advanced-features)
9. [Troubleshooting](#troubleshooting)
10. [Reference](#reference)

## Introduction

RoyaOS is an operating system designed specifically for hosting and running Roya AGI (Artificial General Intelligence). It provides a robust, secure, and efficient environment for AGI operations, with features tailored to the unique requirements of advanced cognitive systems.

### Key Concepts

- **Kernel**: The core of RoyaOS, managing system resources and providing essential services
- **Memory Manager**: Handles allocation, deallocation, and optimization of memory resources
- **Tool Manager**: Manages integration with external tools and APIs
- **Security Module**: Ensures secure operation of the AGI system
- **Interface Layer**: Provides APIs for Roya AGI to interact with the operating system

## Getting Started

### System Overview

RoyaOS runs as a service that provides operating system capabilities to Roya AGI. After installation, it operates in the background, handling resource management, security, and communication with the AGI system.

### Checking System Status

To check if RoyaOS is running correctly:

```bash
curl http://localhost:8000/status
```

This will return a JSON response with system status information, including:
- System uptime
- Memory usage
- Active subsystems
- Connected AGI sessions

### Basic Configuration

The main configuration file is located at `config/config.yaml`. This file contains settings for all aspects of RoyaOS, including:

- System settings
- Memory configuration
- Tool integration
- Security policies

## Basic Operations

### Starting and Stopping RoyaOS

To start RoyaOS:

```bash
cargo run --release
```

To stop RoyaOS, press `Ctrl+C` in the terminal where it's running, or use the shutdown API:

```bash
curl -X POST http://localhost:8000/shutdown
```

### Monitoring System Activity

RoyaOS provides several ways to monitor system activity:

- **Log Files**: Check the `logs` directory for detailed logs
- **Status API**: Use the status endpoint for real-time information
- **Metrics Dashboard**: Access the web dashboard at `http://localhost:8000/dashboard`

### System Commands

RoyaOS supports various system commands that can be executed via the API:

```bash
curl -X POST http://localhost:8000/command -d '{"command": "status"}'
curl -X POST http://localhost:8000/command -d '{"command": "restart"}'
curl -X POST http://localhost:8000/command -d '{"command": "optimize_memory"}'
```

## Memory Management

RoyaOS implements a cognitive memory model designed specifically for AGI operations.

### Memory Categories

Memory in RoyaOS is organized into several categories:

- **System**: Critical system memory
- **ShortTerm**: Immediate cognitive processes
- **Working**: Active processing
- **LongTerm**: Persistent storage
- **Background**: Low-priority memory

### Memory Allocation

Roya AGI can allocate memory through the RoyaOS API:

```bash
curl -X POST http://localhost:8000/memory/allocate -d '{
  "size_bytes": 1048576,
  "purpose": "Image processing",
  "category": "Working"
}'
```

This returns a memory handle that can be used for future operations.

### Memory Optimization

RoyaOS automatically optimizes memory usage based on the configured strategy:

- **Aggressive**: Frequently reclaims unused memory
- **Balanced**: Moderate optimization
- **Conservative**: Minimal intervention

To manually trigger memory optimization:

```bash
curl -X POST http://localhost:8000/memory/optimize
```

## Tool Integration

RoyaOS provides a powerful tool integration system that allows Roya AGI to use external tools and APIs.

### Available Tools

To list available tools:

```bash
curl http://localhost:8000/tools/list
```

### Using Tools

To execute a tool capability:

```bash
curl -X POST http://localhost:8000/tools/execute -d '{
  "tool_id": "calculator",
  "capability": "add",
  "parameters": {"a": 5, "b": 3}
}'
```

### Adding Custom Tools

Custom tools can be added to RoyaOS by:

1. Creating a tool manifest file
2. Implementing the tool interface
3. Placing the tool in one of the configured tool directories

## Security Management

RoyaOS implements a comprehensive security system to ensure safe AGI operations.

### Security Levels

RoyaOS supports multiple security levels:

- **Low**: Minimal restrictions
- **Standard**: Balanced security
- **High**: Strict security
- **Maximum**: Maximum restrictions

To change the security level:

```bash
curl -X POST http://localhost:8000/security/level -d '{"level": "high"}'
```

### Permission Management

Permissions control what operations Roya AGI can perform:

```bash
# Add a permission
curl -X POST http://localhost:8000/security/permissions/add -d '{
  "resource_type": "file",
  "operation": "read",
  "resource": "/data/images/*"
}'

# Remove a permission
curl -X POST http://localhost:8000/security/permissions/remove -d '{
  "resource_type": "file",
  "operation": "write",
  "resource": "/system/*"
}'
```

### Security Auditing

RoyaOS maintains a security audit log that can be accessed:

```bash
curl http://localhost:8000/security/audit?limit=100
```

## Interface and Communication

RoyaOS provides a robust interface for communication with Roya AGI.

### Sessions

AGI sessions are established through the interface:

```bash
# Create a session
curl -X POST http://localhost:8000/interface/sessions/create -d '{
  "metadata": {"client_version": "1.0", "client_name": "Roya AGI"}
}'

# Close a session
curl -X POST http://localhost:8000/interface/sessions/close -d '{
  "session_id": "12345678-1234-1234-1234-123456789012"
}'
```

### Request Processing

Requests from Roya AGI are processed through the interface:

```bash
curl -X POST http://localhost:8000/interface/process -d '{
  "session_id": "12345678-1234-1234-1234-123456789012",
  "request": {
    "id": "req-001",
    "request_type": "memory_allocate",
    "parameters": {
      "size_bytes": 1048576,
      "purpose": "Image processing",
      "category": "Working"
    },
    "timestamp": 1616161616
  }
}'
```

## Advanced Features

### System Hooks

RoyaOS supports hooks that allow custom code to be executed at specific points:

```bash
# Register a hook
curl -X POST http://localhost:8000/hooks/register -d '{
  "hook_point": "pre_memory_allocate",
  "script_path": "/path/to/hook/script.sh"
}'
```

### Performance Profiling

RoyaOS includes performance profiling tools:

```bash
curl http://localhost:8000/profile/memory
curl http://localhost:8000/profile/cpu
curl http://localhost:8000/profile/io
```

### Backup and Restore

To backup the current system state:

```bash
curl -X POST http://localhost:8000/system/backup -d '{
  "backup_path": "/path/to/backup/directory"
}'
```

To restore from a backup:

```bash
curl -X POST http://localhost:8000/system/restore -d '{
  "backup_path": "/path/to/backup/directory"
}'
```

## Troubleshooting

### Common Issues

#### Memory Allocation Failures

If Roya AGI cannot allocate memory:

1. Check the current memory usage: `curl http://localhost:8000/memory/status`
2. Increase the maximum allocation in `config.yaml` if needed
3. Trigger memory optimization: `curl -X POST http://localhost:8000/memory/optimize`

#### Tool Execution Errors

If tool execution fails:

1. Verify the tool is available: `curl http://localhost:8000/tools/list`
2. Check tool permissions: `curl http://localhost:8000/security/permissions/check -d '{"resource_type": "tool", "operation": "execute", "resource": "tool_name"}'`
3. Review the tool execution logs in `logs/tools.log`

#### Communication Issues

If Roya AGI cannot communicate with RoyaOS:

1. Verify RoyaOS is running: `curl http://localhost:8000/status`
2. Check network connectivity
3. Ensure the session is valid: `curl http://localhost:8000/interface/sessions/list`

### Diagnostic Tools

RoyaOS provides several diagnostic tools:

```bash
# System diagnostics
curl http://localhost:8000/diagnostics/system

# Memory diagnostics
curl http://localhost:8000/diagnostics/memory

# Tool diagnostics
curl http://localhost:8000/diagnostics/tools
```

## Reference

### API Reference

The complete RoyaOS API reference is available at `http://localhost:8000/docs` when the system is running.

### Configuration Reference

For a complete reference of configuration options, see the [Configuration Guide](configuration.md).

### System Requirements

For system requirements and optimization recommendations, see the [Installation Guide](installation.md).

### Further Reading

- [Architecture Overview](architecture.md) - Detailed explanation of RoyaOS architecture
- [Developer Guide](developer.md) - Guide for developers extending RoyaOS
- [Security Model](security_model.md) - In-depth explanation of the RoyaOS security model
