/// SVG Icons
///
/// Most icons are copied from <https://heroicons.com/>.
use leptos::*;

#[component]
pub fn CloudArrowDown() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 9.75v6.75m0 0-3-3m3 3 3-3m-8.25 6a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z"
        />
      </svg>
    }
}

#[component]
pub fn CloudArrowUp() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 16.5V9.75m0 0 3 3m-3-3-3 3M6.75 19.5a4.5 4.5 0 0 1-1.41-8.775 5.25 5.25 0 0 1 10.233-2.33 3 3 0 0 1 3.758 3.848A3.752 3.752 0 0 1 18 19.5H6.75Z"
        />
      </svg>
    }
}

#[component]
pub fn DocumentArrowDown() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m.75 12 3 3m0 0 3-3m-3 3v-6m-1.5-9H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"
        />
      </svg>
    }
}

#[component]
pub fn DocumentArrowUp() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m6.75 12-3-3m0 0-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z"
        />
      </svg>
    }
}

#[component]
pub fn Backspace() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 9.75 14.25 12m0 0 2.25 2.25M14.25 12l2.25-2.25M14.25 12 12 14.25m-2.58 4.92-6.374-6.375a1.125 1.125 0 0 1 0-1.59L9.42 4.83c.21-.211.497-.33.795-.33H19.5a2.25 2.25 0 0 1 2.25 2.25v10.5a2.25 2.25 0 0 1-2.25 2.25h-9.284c-.298 0-.585-.119-.795-.33Z"
        />
      </svg>
    }
}

#[component]
pub fn LightBulb() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18"
        />
      </svg>
    }
}

#[component]
pub fn Trash() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
        />
      </svg>
    }
}

#[component]
pub fn Bars3() -> impl IntoView {
    view! {
      <svg
        class="mr-3 h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
        />
      </svg>
    }
}

#[component]
pub fn InformationCircle() -> impl IntoView {
    view! {
      <svg
        aria-haspopup="true"
        class="icon"
        width="20"
        height="20"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="#A0AEC0"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path stroke="none" d="M0 0h24v24H0z" />
        <circle cx="12" cy="12" r="9" />
        <line x1="12" y1="8" x2="12.01" y2="8" />
        <polyline points="11 12 12 12 12 16 13 16" />
      </svg>
    }
}

#[component]
pub fn TheLaend() -> impl IntoView {
    view! {
      <svg
        preserveAspectRatio="meet"
        viewBox="100 100 40 46"
        version="1.1"
        width="40"
        height="46"
      >
        <g
          fill="none"
          fill-rule="evenodd"
          transform="translate(97,99)"
        >
          <path
            d="m18.5 4.79 3.19 3.17a0.494 0.494 0 0 0 0.667 0.029l5.32-4.47a0.701 0.701 0 0 0 0.211-0.305l0.628-1.72a0.702 0.702 0 0 1 0.776-0.477l3.58 0.575a0.725 0.725 0 0 1 0.514 0.368l2.04 3.8a0.684 0.684 0 0 0 0.674 0.379l1-0.063a0.707 0.707 0 0 1 0.707 0.437l5.01 12.1c0.184 0.432 0.206 0.916 0.063 1.36l-2.28 7.09a0.706 0.706 0 0 1-0.348 0.42l-3.01 1.51a0.726 0.726 0 0 0-0.371 0.805l2.85 12.3a0.726 0.726 0 0 1-0.36 0.8l-6.33 3.36a0.732 0.732 0 0 1-0.292 0.081l-2.13 0.115a0.85 0.85 0 0 1-0.21 0l-7.27-1.88a1.4 1.4 0 0 1-0.696-0.414l-1.39-1.52a0.694 0.694 0 0 0-0.485-0.23l-2.53-0.115a0.688 0.688 0 0 0-0.571 0.23l-0.525 0.574a0.381 0.381 0 0 0 0 0.5l1.14 1.35a0.474 0.474 0 0 1-0.08 0.684l-0.97 0.712a0.722 0.722 0 0 1-0.383 0.138l-11.1 0.582a0.7 0.7 0 0 1-0.639-0.345l-1.79-2.99a0.711 0.711 0 0 1-0.103-0.517l3.72-18.1a3.06 3.06 0 0 1 0.68-1.37l6.39-7.31a0.839 0.839 0 0 0 0.166-0.345l1.87-10.2a0.706 0.706 0 0 1 0.423-0.523l1.48-0.638a0.704 0.704 0 0 1 0.776 0.15"
            fill="#ffff00"
          />
          <path
            d="m30.1 32.2v-2.82h0.434c0.898 0 1.14 0.469 1.14 1.41 0 0.938-0.245 1.41-1.14 1.41zm-1.28 0.96h1.9c1.5 0 2.22-1 2.22-2.37 0-1.37-0.726-2.37-2.22-2.37h-1.9l-6e-3 4.73zm-2.65 0h1.35v-4.72h-1.22v2.73h-0.097l-1.54-2.7h-1.36v4.74h1.24v-2.7h0.12l1.54 2.67-0.034-0.017zm-9.58 0v-1.05h-1.93v-3.67h-1.3v4.74l3.23-0.017zm9.72-11v-0.96h-2.94v4.74h3.34v-0.966h-2.05v-0.921h1.65v-0.938h-1.65v-0.938zm-5.48 3.78h1.26v-4.72h-1.25v1.82h-1.46v-1.82h-1.29v4.72h1.28v-1.82h1.46zm-3.97-3.7v-1.02h-3.92v1.04h1.32v3.68h1.29v-3.68zm4.04 4.52a0.575 0.575 0 0 0-0.44 0.169 0.561 0.561 0 0 0-0.171 0.435 0.595 0.595 0 0 0 0.175 0.427 0.61 0.61 0 0 0 0.43 0.177c0.162 0 0.318-0.06 0.435-0.17a0.622 0.622 0 0 0 0-0.864 0.541 0.541 0 0 0-0.429-0.147zm-1.84 0a0.575 0.575 0 0 0-0.44 0.163 0.654 0.654 0 0 0-0.131 0.203 0.498 0.498 0 0 0-0.04 0.238 0.56 0.56 0 0 0 0.04 0.237 0.564 0.564 0 0 0 0.131 0.203c0.117 0.113 0.278 0.17 0.44 0.158a0.568 0.568 0 0 0 0.566-0.362 0.688 0.688 0 0 0 6e-3 -0.475 0.939 0.939 0 0 0-0.131-0.174 0.573 0.573 0 0 0-0.2-0.124 0.502 0.502 0 0 0-0.24-0.04v-0.027zm1.72 1.61h-1.72l-1.44 4.64v0.097h1.32l0.217-0.82h1.52l0.223 0.82h1.32v-0.097l-1.44-4.64zm-1.38 2.98 0.48-1.81h0.097l0.486 1.81h-1.06z"
            fill="#000000"
            fill-rule="nonzero"
          />
        </g>
      </svg>
    }
}

// This one is from <https://www.svgrepo.com>
#[component]
pub fn LinkedIn() -> impl IntoView {
    view! {
      <svg
        class="mr-3 w-6 h-6"
        width="30"
        height="30"
        fill="#ffffff"
        version="1.1"
        viewBox="0 0 30 310"
      >
        <g
          transform="translate(-140 -.341)"
          fill="#fff"
        >
          <path
            d="m72.2 99.7h-62.2c-2.76 0-5 2.24-5 5v200c0 2.76 2.24 5 5 5h62.2c2.76 0 5-2.24 5-5v-200c0-2.76-2.24-5-5-5z"
          />
          <path
            d="m41.1 0.341c-22.6 0-41.1 18.4-41.1 41 0 22.6 18.4 41 41.1 41 22.6 0 41-18.4 41-41 1e-3 -22.6-18.4-41-41-41z"
          />
          <path
            d="m230 94.8c-25 0-43.5 10.7-54.7 23v-13c0-2.76-2.24-5-5-5h-59.6c-2.76 0-5 2.24-5 5v200c0 2.76 2.24 5 5 5h62.1c2.76 0 5-2.24 5-5v-98.9c0-33.3 9.05-46.3 32.3-46.3 25.3 0 27.3 20.8 27.3 48v97.2c0 2.76 2.24 5 5 5h62.1c2.76 0 5-2.24 5-5v-110c0-49.6-9.45-100-79.5-100z"
          />
        </g>
      </svg>
    }
}

#[component]
pub fn ExternalLink() -> impl IntoView {
    view! {
      <svg
        width="20"
        height="25"
        viewBox="0 0 80 40"
        style = "display: inline; position: relative; top: -5px; left: 5px; margin-right: 5px;"
      >
      <path d="M48 26c-1.1 0-2 0.9-2 2v26H10V18h26c1.1 0 2-0.9 2-2s-0.9-2-2-2H8c-1.1 0-2 0.9-2 2v40c0 1.1 0.9 2 2 2h40c1.1 0 2-0.9 2-2V28C50 26.9 49.1 26 48 26z"></path><path d="M56 6H44c-1.1 0-2 0.9-2 2s0.9 2 2 2h7.2L30.6 30.6c-0.8 0.8-0.8 2 0 2.8C31 33.8 31.5 34 32 34s1-0.2 1.4-0.6L54 12.8V20c0 1.1 0.9 2 2 2s2-0.9 2-2V8C58 6.9 57.1 6 56 6z"></path>
      </svg>
    }
}
