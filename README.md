## Card game
This is repo for my yet to be named roguelite cardgame.

## Setup
Only the client and server can not be run directly after having the project cloned. Both projects have a README.md that explain what needs to be done in order to set them up.

### Why not a cargo workspace?
There are some dependencies of dependencies for both the client and the server that can depend on async-std or tokio but will fail to compile if both features are enabled. This causes compile problems when workspaces are used because the client uses the async-std feature while the server uses tokio.
