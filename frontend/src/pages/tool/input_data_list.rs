use std::{collections::HashMap, hash::Hash, rc::Rc};

use leptos::*;

use crate::forms::{FieldSet, FieldSignal};

#[component]
pub fn InputDataList<'a, ID>(
    field_sets: &'a [FieldSet<ID>],
    signals: &'a Rc<HashMap<ID, FieldSignal>>,
) -> impl IntoView
where
    ID: Hash + Eq + AsRef<str> + Copy + 'static,
{
    let sets = field_sets.iter().map(|fs|{
        let values: Vec<_> = fs.fields.iter().map(|field|{
            let signal: Option<FieldSignal> = signals.get(&field.id).copied();
            view! {
              <dt class="font-semibold text-right px-3 py-1 text-gray-500">{ field.label }</dt>
              <dd class="py-1 px-3">
                {
                  move || signal.and_then(|s|s.as_formatted_string()).unwrap_or_else(||"-".to_string())
                }
                <span class="ml-2 text-gray-400">{ field.unit() }</span>
              </dd>
            }
        }).collect();

        view!{
          <li class="px-3">
            <div class="font-semibold text-lg border-solid border-b text-gray-400">
              { fs.title }
            </div>
            <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
              { values }
            </dl>
          </li>
        }
    }).collect::<Vec<_>>();

    view! {
      <ul class="grid grid-cols-3">
        { sets }
      </ul>
    }
}
