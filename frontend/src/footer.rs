use leptos::*;

const LOGO: &str = include_str!("../assets/logo_BMWK_NKI.svg");
const FOOTER_MD: &str = include_str!("../content/footer.md");
const WIKI_URL: &str = "https://codeberg.org/slowtec/klick/wiki";
const DATENSCHUTZ: &str = "https://www.umwelttechnik-bw.de/de/datenschutz";
const ACCESSIBILITY: &str = "https://www.umwelttechnik-bw.de/de/erklaerung-barrierefreiheit";
const LINKEDIN: &str = "https://www.linkedin.com/company/umwelttechnik-bw";
const FAQ: &str = "https://codeberg.org/slowtec/klick/wiki/FAQ";

use klick_app_components::icons;

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
        <icons::TheLaend />
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
