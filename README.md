# Rabbit-Borough
## A rabbit MQ abstraction build upon lapin

Goal: To fit my needs and hopefully, someone else's

(work in progress) my plan is to add a bit of code at least 2-4 times per week (until I reach a satisfying point)

## HOW TO USE

* the repo was restructured into a library structure

examples can be found under /examples

at this point there is only one example and it can be executed as follows  `cargo run --example consumer`

## IDEA

The whole idea is basically to be able to create a consumer project with minimal effort, by bypassing templating, configuration and complicated resiliency logic. 

The only thing, that will eventually needed is to write the message handler logic 