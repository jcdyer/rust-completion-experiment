# Rust completion experiment.

This is an experiment to see what open-craft/openedx-completion-aggregator
would look like in hexagonal architecture, and what hexagonal architecture
looks like in Rust.

In short, the code calculates what percentage of a course, or a section of a
course users have completed.  To do this, it needs to know:

1.  Who is enrolled in the course.
2.  How the course is structured.  (Each course is a tree or DAG, of XBlocks,
    where some XBlocks contain other XBlocks, and other XBlocks contain course
    content.) 
3.  Which individual XBlocks have been completed by each enrolled learner. 

It then exposes an interface to allow clients to 

1.  Trigger aggregation of a user's completion in a course.
2.  Retrieve completion data for one or more users in the course.
3.  Retrieve completion data for all users in one course.


## Design:

This project uses hexagonal architecture (a.k.a. ports-and-adapters).  I've
implemented this as simply as possible, using tools provided natively by the
Rust programming language.  

According to the ports and adapters architecture, client interactions come 
through the "primary ports."  These will simply be modeled as the public 
methods of the App object, possibly supported by some documented public types. 

Primary adapters will be separate interfaces, such as a web server or a CLI
that can call into these ports.  It may be sufficient to have a binary for each
primary adapter that depends on the core business logic.  There can be many
adapters using each primary port.

Secondary ports are the backends that provide the data the app needs.  They are
modeled as traits that provide data access methods.  Each trait can be
implemented by different data repositories, but each one will have a single
implementation in effect at a time.

Secondary adapters will be implemented as types that implement the secondary 
port traits.  There could be a Postgres implementation, a mongodb 
implementation, an external REST API, (how to pass user tokens from the primary 
ports?) or an in-memory data structure for testing.  When the App is 
instantiated, it will take the secondary adapters (or services) as arguments
to its constructor, as a form of simple dependency injection.
