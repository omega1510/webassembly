use yew::prelude::*;

struct Model {
    counter: i32,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model { counter: 0 });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Model {
                counter: state.counter + 1,
            });
        })
    };

    html! {

        <div class="container">
            <br/>
            <div class="row justify-content-center">

                <button type="button" class="btn btn-primary" {onclick}>
                    { "Increment" }
                </button>

                <div class="col-md-4">
                    <div class="card">
                        <div class="card-header">
                            <h1>
                                { "Counter" }
                            </h1>
                        </div>
                        <div class="card-body">
                            <h2>
                                { state.counter }
                            </h2>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
