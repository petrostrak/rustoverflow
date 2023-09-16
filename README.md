## RustOverflow
An API for a StackOverflow-like app.

Two primary features in StackOverflow will be implemented:

*  Question creation, retrieval
*  Answer creation, retrieval & deletion

### Objective

In this project, we aim to learn and practice the following:

*  Designing & building APIs
*  Using a backend framework (Rocket)
*  Designing SQL models
*  Hands-on usage of Postgres
*  Writing testable code
*  Organizing code using modules
*  Navigating and contributing to an existing code base

First, install `sqlx-cli`. This is SQLx's associated command-line utility for managing databases, migrations, and more:
```
cargo install sqlx-cli  
```

Now you can create migrations by running:
```
sqlx migrate add <name> -r  
```

And execute migrations by running:
```
sqlx migrate run  
```

NOTE: If you ever want to revert the migrations, simply run:
```
sqlx migrate revert  
```