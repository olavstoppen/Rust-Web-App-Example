// Import the necessary Yew components for creating the web application.
use yew::prelude::*;

// Define a message enum to represent different actions in the app.
enum Msg {
    AddOne,
}

// Define the App struct which will hold the state of the application (in this case, the counter).
struct App {
    count: i64,
}

// Implement the Component trait for the App struct.
impl Component for App {
    type Message = Msg;
    type Properties = ();

    // Create the initial state of the app with a counter set to 0.
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count: 0,
        }
    }

    // Update the app state based on incoming messages.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
             // If the message is AddOne, increment the counter and return true to re-render the component.
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
        }
    }

     // Render the view for the app, including the counter and a button to increment it.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

// The main function starts the Yew application.
fn main() {
    // Create a new Renderer instance and render the App component.
    // The app will use the index.html and style.css files located at the root directory.
    // To start the app, run the command: trunk serve
    // This will start a server and render the application on http://localhost:8080.
    
    yew::Renderer::<App>::new().render();
}