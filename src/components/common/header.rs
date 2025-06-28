use leptos::prelude::*;

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
      <header class="flex items-center justify-between text-6x1 tracking-wide">
      <div>
        <h1>Welcome to my page!!!</h1>
      </div>
      <label class="flex cursor-pointer gap-2 items-center">
        <span class="w-4 h-4 rounded-full bg-black inline-block"></span>
        <input type="checkbox" value="cupcake" class="toggle theme-controller" />
        <span class="w-4 h-4 rounded-full bg-white border border-gray-400 inline-block"></span>
      </label>
      </header>
    }
}
