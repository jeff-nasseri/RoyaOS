Feature: Basic RoyaOS Operations
  As a user of RoyaOS
  I want to perform basic system operations
  So that I can verify the system is functioning correctly

  Background:
    Given RoyaOS is running
    And Roya AGI is connected to the system

  Scenario: System Initialization
    When the system initializes
    Then the kernel should be loaded
    And the memory manager should be initialized
    And the tool manager should be available
    And the security module should be active
    And the interface layer should be ready

  Scenario: Memory Management
    When Roya AGI requests memory allocation of 1GB
    Then the memory manager should allocate 1GB of memory
    And return a memory handle to Roya AGI
    When Roya AGI releases the memory handle
    Then the memory manager should deallocate the memory

  Scenario: Tool Integration
    Given a "calculator" tool is available in the system
    When Roya AGI requests to use the "calculator" tool
    Then the tool manager should provide access to the "calculator" tool
    And Roya AGI should be able to perform calculations using the tool

  Scenario: Security Enforcement
    When Roya AGI attempts to access restricted system resources
    Then the security module should evaluate the request
    And either grant or deny access based on security policies
    And log the access attempt

  Scenario: System Shutdown
    When a system shutdown is requested
    Then RoyaOS should notify all components to prepare for shutdown
    And save necessary state information
    And terminate all processes in an orderly manner
    And power down the system
