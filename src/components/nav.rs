use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    let menu = "Menu";
    let about = "About";

    html! {
      <div class="relative">
          <button type="button" class="inline-flex items-center gap-x-1 text-sm font-semibold leading-6 text-gray-900" aria-expanded="false">
            <span>{menu}</span>
          </button>

        <div class="absolute left-1/2 z-10 mt-5 flex w-screen max-w-max -translate-x-1/2 px-4">
          <div class="w-screen max-w-md flex-auto overflow-hidden rounded-3xl bg-white text-sm leading-6 shadow-lg ring-1 ring-gray-900/5">
            <div class="p-4">
              <div class="group relative flex gap-x-6 rounded-lg p-4 hover:bg-gray-50">
                <div class="mt-1 flex h-11 w-11 flex-none items-center justify-center rounded-lg bg-gray-50 group-hover:bg-white">
                </div>
                <div>
                  <a href="#" class="font-semibold text-gray-900">
                    {about}
                    <span class="absolute inset-0"></span>
                  </a>
                  <p class="mt-1 text-gray-600"></p>
                </div>
              </div>
            </div>
          </div>
        </div>
        </div>
    }
}