use leptos::*;

const LOGO: &str = include_str!("../assets/logo_BMWK_NKI.svg");
const FOOTER_MD: &str = include_str!("../content/footer.md");
const WIKI_URL: &str = "https://codeberg.org/slowtec/klick/wiki";
const DATENSCHUTZ: &str = "https://www.umwelttechnik-bw.de/de/datenschutz";
const ACCESSIBILITY: &str = "https://www.umwelttechnik-bw.de/de/erklaerung-barrierefreiheit";
const LINKEDIN: &str = "https://www.linkedin.com/company/umwelttechnik-bw";
const FAQ: &str = "https://codeberg.org/slowtec/klick/wiki/FAQ";

use crate::{Markdown, Page, CHANGELOG_URL, VERSION};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer class="bg-gray-100">
        <div class="mx-auto max-w-7xl overflow-hidden px-6 pt-16 sm:pt-20 lg:px-8">
          <div class="mb-14">
            <div class="text-gray-600 text-center grid place-content-center">
              <div class="grid place-content-center mb-8" inner_html = LOGO></div>
              <Markdown content = FOOTER_MD />
            </div>
          </div>
          <div>
            <div class="mt-10 flex justify-center space-x-10">
              <a href="https://codeberg.org/slowtec/klick" class="text-gray-400 hover:text-gray-500">
                <span class="sr-only">Codeberg</span>
                <svg aria-hidden="true" viewBox="-2 -2 28 28" class="h-6 w-6 fill-slate-500 group-hover:fill-slate-700">
                  <path d="M11.955.49A12 12 0 0 0 0 12.49a12 12 0 0 0 1.832 6.373L11.838 5.928a.187.14 0 0 1 .324 0l10.006 12.935A12 12 0 0 0 24 12.49a12 12 0 0 0-12-12 12 12 0 0 0-.045 0zm.375 6.467 4.416 16.553a12 12 0 0 0 5.137-4.213z"></path>
                </svg>
              </a>
            </div>
            <p class="mb-8 mt-4 text-center text-xs leading-5 text-gray-400">
              <a class="no-underline hover:underline" href= { CHANGELOG_URL} >"v" { VERSION }</a>
            </p>
            <p class="mb-8 text-center text-xs leading-5 text-gray-500">
              "Made with ♥ by "
              <a href="https://slowtec.de">"slowtec GmbH"</a>
            </p>
          </div>

          </div>
        <div>
          <nav class="sm:flex flex h-16 sm:flex sm:items-center justify-between sm:justify-center sm:space-x-12 bg-black" aria-label="Footer">
          <FooterLink link = LinkType::Page(Page::Home) label ="KlicK" />
          <FooterLink link = LinkType::Page(Page::Tool) label ="Tool" />
          <FooterLink link = LinkType::External(DATENSCHUTZ) label ="Datenschutz" />
          <FooterLink link = LinkType::Page(Page::Faq) label ="FAQs" />
          <FooterLink link = LinkType::External(ACCESSIBILITY) label ="Barrierefreiheit" />
          <FooterLink link = LinkType::External(WIKI_URL) label ="Wiki" />
          <FooterLink link = LinkType::Page(Page::OpenSource) label ="Open Source" />
          <FooterLink link = LinkType::Page(Page::Imprint) label ="Impressum" />
          <LinkedIn />
          <TheLand />
          </nav>
        </div>
      </footer>
    }
}

#[component]
fn FooterLink(link: LinkType, label: &'static str) -> impl IntoView {
    view! {
      <div class="">
        <a
          href = {
            match link {
              LinkType::Page(page) => page.path(),
              LinkType::External(url) => url
            }
          }
          class = "text-sm leading-6 text-white hover:text-gray-200">
          { label }
        </a>
      </div>
    }
}

#[component]
fn TheLand() -> impl IntoView {
    view! {
      <div>
      <a href="https://www.thelaend.de">
      <svg
        preserveAspectRatio="meet"
        viewBox="100 100 40.000084 46.000282"
        version="1.1"
        id="svg8"
        width="40.000084"
        height="46.000282"
        inkscape:version="1.0.1 (3bc2e813f5, 2020-09-07)">

        <defs id="defs12" />
        <g
          fill="none"
          fill-rule="evenodd"
          id="g6"
          transform="translate(96.999889,98.999397)">
          <path
            d="m 18.521,4.786 3.186,3.173 a 0.494,0.494 0 0 0 0.667,0.029 L 27.689,3.515 A 0.701,0.701 0 0 0 27.9,3.21 l 0.628,-1.724 a 0.702,0.702 0 0 1 0.776,-0.477 l 3.585,0.575 a 0.725,0.725 0 0 1 0.514,0.368 l 2.043,3.8 a 0.684,0.684 0 0 0 0.674,0.379 l 1.005,-0.063 a 0.707,0.707 0 0 1 0.707,0.437 l 5.012,12.072 c 0.184,0.432 0.206,0.916 0.063,1.362 l -2.283,7.089 a 0.706,0.706 0 0 1 -0.348,0.42 l -3.014,1.511 a 0.726,0.726 0 0 0 -0.371,0.805 l 2.854,12.268 a 0.726,0.726 0 0 1 -0.36,0.8 l -6.33,3.356 a 0.732,0.732 0 0 1 -0.292,0.081 l -2.129,0.115 a 0.85,0.85 0 0 1 -0.21,0 l -7.268,-1.88 A 1.402,1.402 0 0 1 22.46,44.09 l -1.393,-1.517 a 0.694,0.694 0 0 0 -0.485,-0.23 l -2.534,-0.115 a 0.688,0.688 0 0 0 -0.571,0.23 l -0.525,0.574 a 0.381,0.381 0 0 0 0,0.5 l 1.141,1.352 a 0.474,0.474 0 0 1 -0.08,0.684 l -0.97,0.712 A 0.722,0.722 0 0 1 16.66,46.418 L 5.546,47 A 0.7,0.7 0 0 1 4.907,46.655 L 3.115,43.665 A 0.711,0.711 0 0 1 3.012,43.148 L 6.734,25.057 a 3.06,3.06 0 0 1 0.68,-1.368 l 6.392,-7.307 a 0.839,0.839 0 0 0 0.166,-0.345 L 15.838,5.798 a 0.706,0.706 0 0 1 0.423,-0.523 l 1.484,-0.638 a 0.704,0.704 0 0 1 0.776,0.15"
            fill="#ffff00"
            id="path2" />
          <path
            d="M 30.148,32.194 V 29.37 h 0.434 c 0.898,0 1.143,0.469 1.143,1.406 0,0.938 -0.245,1.407 -1.143,1.407 z m -1.275,0.96 h 1.904 c 1.497,0 2.223,-1 2.223,-2.366 0,-1.368 -0.726,-2.367 -2.223,-2.367 h -1.898 l -0.006,4.734 z m -2.646,0 h 1.349 v -4.722 h -1.218 v 2.729 h -0.097 l -1.543,-2.7 H 23.363 V 33.2 h 1.235 v -2.7 h 0.12 l 1.543,2.672 -0.034,-0.017 z m -9.58,0 v -1.05 h -1.932 v -3.672 h -1.298 v 4.74 l 3.23,-0.017 z M 26.364,22.2 v -0.96 h -2.938 v 4.74 h 3.344 v -0.966 h -2.052 v -0.921 h 1.652 v -0.938 h -1.652 v -0.938 z m -5.476,3.78 h 1.257 v -4.723 h -1.251 v 1.825 h -1.458 v -1.825 h -1.292 v 4.723 h 1.28 v -1.82 h 1.458 z m -3.967,-3.7 V 21.257 H 13 v 1.04 h 1.32 v 3.683 h 1.286 v -3.684 z m 4.041,4.52 a 0.575,0.575 0 0 0 -0.44,0.169 0.561,0.561 0 0 0 -0.171,0.435 0.595,0.595 0 0 0 0.175,0.427 0.61,0.61 0 0 0 0.43,0.177 c 0.162,0 0.318,-0.06 0.435,-0.17 a 0.622,0.622 0 0 0 0,-0.864 0.541,0.541 0 0 0 -0.429,-0.147 z m -1.835,0 a 0.575,0.575 0 0 0 -0.44,0.163 0.654,0.654 0 0 0 -0.131,0.203 0.498,0.498 0 0 0 -0.04,0.238 0.56,0.56 0 0 0 0.04,0.237 0.564,0.564 0 0 0 0.131,0.203 c 0.117,0.113 0.278,0.17 0.44,0.158 a 0.568,0.568 0 0 0 0.566,-0.362 0.688,0.688 0 0 0 0.006,-0.475 0.939,0.939 0 0 0 -0.131,-0.174 0.573,0.573 0 0 0 -0.2,-0.124 0.502,0.502 0 0 0 -0.24,-0.04 V 26.8 Z m 1.715,1.61 h -1.715 l -1.44,4.637 v 0.097 h 1.315 l 0.217,-0.82 h 1.52 l 0.223,0.82 h 1.315 v -0.097 l -1.435,-4.638 z m -1.383,2.982 0.48,-1.813 h 0.097 l 0.486,1.813 H 19.46 Z"
            fill="#000000"
            fill-rule="nonzero"
            id="path4" />
        </g>
      </svg>
      </a>
      </div>
    }
}

#[component]
fn LinkedIn() -> impl IntoView {
    view! {
      <div>
      <a href={ LINKEDIN }>
      //<!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->
      <svg fill="white" height="30px" version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 310 310" xml:space="preserve">
      <g id="XMLID_801_">
        <path id="XMLID_802_" d="M72.16,99.73H9.927c-2.762,0-5,2.239-5,5v199.928c0,2.762,2.238,5,5,5H72.16c2.762,0,5-2.238,5-5V104.73   C77.16,101.969,74.922,99.73,72.16,99.73z"/>
        <path id="XMLID_803_" d="M41.066,0.341C18.422,0.341,0,18.743,0,41.362C0,63.991,18.422,82.4,41.066,82.4   c22.626,0,41.033-18.41,41.033-41.038C82.1,18.743,63.692,0.341,41.066,0.341z"/>
        <path id="XMLID_804_" d="M230.454,94.761c-24.995,0-43.472,10.745-54.679,22.954V104.73c0-2.761-2.238-5-5-5h-59.599   c-2.762,0-5,2.239-5,5v199.928c0,2.762,2.238,5,5,5h62.097c2.762,0,5-2.238,5-5v-98.918c0-33.333,9.054-46.319,32.29-46.319   c25.306,0,27.317,20.818,27.317,48.034v97.204c0,2.762,2.238,5,5,5H305c2.762,0,5-2.238,5-5V194.995   C310,145.43,300.549,94.761,230.454,94.761z"/>
      </g>
      </svg>
      </a>
      </div>
    }
}

#[derive(Clone, Copy)]
enum LinkType {
    Page(Page),
    External(&'static str),
}
