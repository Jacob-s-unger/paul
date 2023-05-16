# paul
Pocket Mechanic - digitized maintainence records for vehicle projects

### Proposed Application Structure/Stack

- Front End: React Native. Will help support both iOS and AndroidOS. Will be written in JavaScript, which can be easily integrated with Rust Backend using WebAssembly

- Back End: Rust. This project was initially conceived to learn rust better. Goal is to also build an API to the backend using Actix-Web to learn more about RESTful API builidng and execution

- Database: SQLite. This was chosen because its lightweight, serverless, and a self contained database engine that is easy to set up and integrate with Rust using the 'rusqlite' crate. This is suitible for small scale application such as the one goaled here but will need to be revisited if the app grows.

- Rust JavaScript Integration: WebAssembly (Wasm). This is a binary instruction format that allows you to run Rust code alongside JavaScript in a web enviroment. Using 'wasm-bindgen' tool, can generate the necessary bindings between the two frameworks

- Version Control: Git and Github! 

- Deployment and Hosting: AWS or Heroku for Rust Deployment, likely AWS as a way to continue learning about AWS Hosting and security controls, as thats what I focus on for work. For the FrontEnd, we will use services such as Expo or React Native CLI to build and distribute to AppStore and Google Play store if we ever make it that far!