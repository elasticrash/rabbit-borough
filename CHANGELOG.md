### 0.1.5

* Added publish_with_type so as to be able to publish messages of a specific type

### 0.1.4

* Tested thoroughly in one of my personal projects
* Added example on how to use postgres and r2d2 in a consumer project

### 0.1.3-beta.5

* Added consume_with_option that allows to pass an Option of type T into the handler

### 0.1.3-beta.4

* Wrap lapin Delivery as DeliveredMessage so as not to need to include lapin as a direct dependency
* Replaced GetConnectionError with GenericError<T>
* Replaced ConnectionState with ErrorType

### 0.1.3-beta.3

* Added a publisher module and publisher example.
* Create_channel and build_url are now publish and got moved on connection_manager
* Documentation for publisher

### 0.1.3-beta.2

* Added ExchangeOptions in JSONConfiguration
* Improved README and CRATE documents

### 0.1.3-beta

#### Breaking changes

* Renamed handler_message_result to handle_message_result as it was an annoying typo

#### Bug fixes and general improvements

* Added better descriptions in documentation
* Went through some manual e2e resiliency testing.

### 0.1.2-alpha

* Added unit tests
* Converted the project structure to a lib
* If the configuration file cannot be read, use the default values
* Removing some logs and replacing them with proper error handling and error propagation
* Added Connection resiliency. If the RabbitMq dies, it will start retrying to connect until it exhausts all retries

### 0.1.1-alpha

* added default on JSONConfiguration
* added DeclareProperties, so as to choose which action should be performed during setup (default to true)
* removed ConsumerConfiguration, as it was considered duplication
* abstracted consumer

### 0.1.0-alpha

* updated to Lapin v1.0.0
* added default implementation for JSONConfiguration properties
* connection retry config
* added ConsumerConfiguration, ConnectionProperties and BindingProperties
