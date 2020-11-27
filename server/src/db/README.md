# Database

Only PostgreSQL is supported by oxidized-CMS at the moment. There are no plans to add NoSQL (MongoDB…) support though MySQL support may come sometime in the far future.

[Diesel](https://diesel.rs/) is used for any direct database Queries/ Updates/ Inserts and Migrations. Pool management is done with [r2d2](https://github.com/sfackler/r2d2) with the [diesel-r2d2](https://docs.diesel.rs/master/diesel/r2d2/index.html) extension.
