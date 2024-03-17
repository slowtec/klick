use leptos::*;

#[component]
pub fn Breadcrumbs<E>(entries: &'static [(&'static str, E)], current: RwSignal<E>) -> impl IntoView
where
    E: Copy + PartialEq + 'static,
{
    let crumbs: Vec<_> = entries
        .iter()
        .copied()
        .map(|(title, entry)| view! { <Entry title entry current /> })
        .collect();

    view! {
      <nav class="flex" aria-label="Breadcrumb">
        <ol role="list" class="flex space-x-4 rounded-md bg-white px-6 shadow">
          { crumbs }
        </ol>
      </nav>
    }
}

#[component]
fn Entry<E>(title: &'static str, entry: E, current: RwSignal<E>) -> impl IntoView
where
    E: Copy + PartialEq + 'static,
{
    view! {
      <li class="flex">
        <div class="flex items-center">
          <svg
            class="h-full w-6 flex-shrink-0 text-gray-200"
            viewBox="0 0 24 44"
            preserveAspectRatio="none"
            fill="currentColor"
            aria-hidden="true"
          >
            <path d="M.293 0l22 22-22 22h1.414l22-22-22-22H.293z" />
          </svg>
          <a
            href="#"
            class= move ||
              if current.get() == entry {
                "ml-4 text-sm font-bold text-black hover:text-gray-700"
              } else {
                "ml-4 text-sm font-medium text-gray-500 hover:text-gray-700"
              }
            aria-current="page"
            on:click = move |_| current.set(entry)
          >
            { title }
          </a>
        </div>
      </li>
    }
}
