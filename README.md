# pals-pal
## Installation
To build it you need to install: rust compiler, cargo package manager and
wasm-pack (for ease of use). 

Command for building the rust code (you need to be inside `pals-pal` directory):
`wasm-pack build --target web`.


Files needed by the web page are located in `pkg/`.
Don't move them, `index.html` gets them from that location.
