/// This module allows you to configure your consumer
/// * The optional reader
/// * The model for setting up your project configuration
pub mod configuration;

/// This module has everything you need to create a consumer
/// * Connection Manager (connection_manager)
/// * AMQP Setup (setup)
/// * Setting up a consumer (consumer)
/// * Handling the outcome of a Message
pub mod consumer;

/// This module has everything you need to create a publisher
/// The publisher creates a one use connection for each message 
/// after sending a message the connection is getting disposed
pub mod publisher;