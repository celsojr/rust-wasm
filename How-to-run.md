## Running rust wasm on the web

### Instructions on how to run your firt rust wasm web app

Assuming you have `rustc 1.72.1 (d5c2e9c34 2023-09-13)` or above already installed on your machine, do the following:

1. Install the `cargo wasm-pack` in case this is not yet installed
```
cargo install wasm-pack
```
2. Install the `cargo-generate` template generator in case this is not yet installed
```
cargo install cargo-generate
```
3. Create your first project with the template generator just installed
```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```
4. Build the wasm files and dependencies. This will create a folder called pkg in the root directory with the actual files.
```
wasm-pack build --target web
```
5. Create an `index.html` file in the root folder and change it according to the file provided along with this repo as an example. This is important to rename this files as `index.html` so that you don't need to worry about extra configurations for you _http_ server in the next step.

6. Last but not least, you need to use an _http_ server to serve the folder where `index.html` resides. This is necessary so that the module script type can run correctly. Otherwise, the script may be ignored by your browser.

Here I will be giving an example with Python, but you feel free to choose any http server you want.
```
python3 -m http.server 3001
```

Now, you can access `http://localhost:3001` from your preferred browser.