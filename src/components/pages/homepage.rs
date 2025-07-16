use leptos::prelude::*;

#[island]
pub fn Button() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <div>
            <button class="btn btn-primary my-4" on:click=move |_| *count.write() += 1>
                Click me
                {count}
            </button>

        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container gap-6 grid grid-cols-1 pt-20 mx-auto text-center">

            <h1 class="text-6xl tracking-wide">"Welcome to my page!"</h1>
            <Button />
            <div>
                <button class="btn" onclick="my_modal_1.showModal()">
                    open modal
                </button>
                <dialog id="my_modal_1" class="modal">
                    <div class="modal-box">
                        <h3 class="text-lg font-bold">Hello!</h3>
                        <p class="py-4">Press ESC key or click the button below to close</p>
                        <div class="modal-action">
                            <form method="dialog">
                                <button class="btn">Close</button>
                            </form>
                        </div>
                    </div>
                </dialog>
            </div>
        </div>
    }
}
