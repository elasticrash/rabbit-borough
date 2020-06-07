### 0.1.3-beta.2

* Added ExchangeOptions in JSONConfiguration

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
