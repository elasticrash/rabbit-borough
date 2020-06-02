
### 0.1.2

* Removing println! and replacing them with proper error handling and error propagation
* Added Connection resiliency. If the RabbitMq dies, it will start retrying to connect until it exhausts all retries

### 0.1.1

* added default on JSONConfiguration
* added DeclareProperties, so as to choose which action should be performed during setup (default to true)
* removed ConsumerConfiguration, as it was considered duplication
* abstracted consumer

### 0.1.0

* updated to Lapin v1.0.0
* added default implementation for JSONConfiguration properties
* connection retry config
* added ConsumerConfiguration, ConnectionProperties and BindingProperties