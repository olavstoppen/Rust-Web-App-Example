# Rust Web App Example | Yew, WASM and trunc

A simple counter app built using the Yew crate in Rust. This application demonstrates the use of Yew and WebAssembly to create a web application with a button and counter.

## 1. Setup

To run the app, you need to install the WebAssembly target and Trunk:

1. Add the WebAssembly target to your Rust installation:

```cli
rustup target add wasm32-unknown-unknown
```

2. Install Trunk:

```cli
cargo install trunk
```

3. In your `Cargo.toml` file, add the Yew crate with version 0.20.0:

```toml
yew = { version = "0.20.0", features = ["csr"] }
```

## 1. Building the app

This app is built using the Yew framework. The core concept of Yew is the Component trait, which allows you to create reusable UI components with their own state and behavior. Components are updated based on incoming messages, allowing you to build complex applications with interactive features.

## 3. Running the App

To run the app, use the following command:

```cli
trunk serve
```

This command generates the dist folder containing the necessary files and runs the app on localhost:8080.

## 4. Styling the App

To style the app, create a style.css file in the root directory. Include this file in your index.html by adding the following line:

```html
<link data-trunk rel="css" href="style.css" />
```

Make sure to include the data-trunk attribute to ensure that Trunk processes the link correctly.

Now you can add your custom styles to the style.css file, and they will be applied to the app.