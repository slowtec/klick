use leptos::*;

fn example_data() -> klick_boundary::FormData {
    klick_boundary::FormData {
        name: "Lingen".to_string(),
        ew: "120000".to_string(),
        flow: "5000000".to_string(),
        csb_zu: "1045".to_string(),
        tkn_zu: "122".to_string(),
        p_zu: "9,9".to_string(),
        csb_ab: "129".to_string(),
        tkn_ab: "11,76".to_string(),
        p_ab: "0,4".to_string(),
        klaergas: "1260000".to_string(),
        methangehalt: "23".to_string(),
        gas_zusatz: "1300000".to_string(),
        biogas: Some("no".to_string()),
        strombedarf: "2683259".to_string(),
        eigenstrom: "2250897".to_string(),
        ef_strommix: "468".to_string(),
        schlammtaschen: Some("yes".to_string()),
        schlammstapel: Some("yes".to_string()),
        klaerschlamm_enstorgung: "3687,6".to_string(),
        klaerschlamm_transport: "47".to_string(),
        betriebsstoffe_fe3: "0".to_string(),
        betriebsstoffe_feso4: "326000".to_string(),
        betriebsstoffe_kalk: "326260".to_string(),
        betriebsstoffe_poly: "23620".to_string(),
        n2o_szenario: "3".to_string(),
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let klick_boundary::FormData {
        name,
        ew,
        flow,
        csb_zu,
        tkn_zu,
        p_zu,
        csb_ab,
        tkn_ab,
        p_ab,
        klaergas,
        methangehalt,
        gas_zusatz,
        biogas,
        strombedarf,
        eigenstrom,
        ef_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_enstorgung,
        klaerschlamm_transport,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
        n2o_szenario,
    } = example_data();

    view! {

    <form action="/submit" method="post" class="space-y-12">

      // Allgemeine Infos zur Kläranlage

      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">Angaben zur Kläranlage</h3>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          // Name der Kläranlage
          <TextInput
            label= "Name oder Ort"
            name="name"
            placeholder="Name der Kläranlage"
            value = name
          />

          // EW-Werte
          <NumberInput
            label= "Ausbaugröße"
            name="ew"
            placeholder="Ausbaugröße [EW]"
            value = ew
            unit = "EW"
          />

          // Abwassermenge
          <NumberInput
            label= "Abwassermenge"
            name="flow"
            placeholder="Abwassermenge"
            value = flow
            unit = "m³/a"
          />

        </div>
      </div>

      // Zulauf-Parameter
      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">"Zulauf-Parameter (Jahresmittelwerte)"</h3>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          // CSB
          <NumberInput
            label = "Chemischer Sauerstoffbedarf"
            name="csb_zu"
            placeholder="CSB"
            value = csb_zu
            unit = "mg/L"
          />

          // TKN
          <NumberInput
            label="Gesamtstickstoff"
            name="tkn_zu" placeholder="TKN"
            value = tkn_zu
            unit = "mg/L"
          />

          // P
          <NumberInput
            label="Phosphor"
            name="p_zu"
            placeholder="P"
            value = p_zu
            unit = "mg/L"
          />

        </div>
      </div>

      // Ablauf-Parameter
      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">"Ablauf-Parameter (Jahresmittelwerte)"</h3>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          // CSB
          <NumberInput
            label = "Chemischer Sauerstoffbedarf"
            name="csb_ab"
            placeholder="CSB"
            value = csb_ab
            unit = "mg/L"
          />

          // TKN
          <NumberInput
            label = "Gesamtstickstoff"
            name="tkn_ab"
            placeholder="TKN"
            value = tkn_ab
            unit = "mg/L"
          />

          // P
          <NumberInput
            label="Phosphor"
            name="p_ab"
            placeholder="P"
            value = p_ab
            unit = "mg/L"
          />

        </div>
      </div>

      // Energiebedarf

      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">Energiebedarf</h3>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
          // Klärgas erzeugt
          <NumberInput
            label="Erzeugtes Klärgas"
            name="klaergas"
            placeholder="Klärgas"
            value = klaergas
            unit = "m³"
          />
          <NumberInput
            label="Methangehalt"
            unit = "%"
            name="methangehalt"
            placeholder="65" value = methangehalt
          />

          // Erdgas zugekauft
          <NumberInput
            label= "Gasbezug (Versorger)"
            name="gas_zusatz"
            placeholder="Gasbezug"
            value = gas_zusatz
            unit = "kWh/a"
          />

          // Biogas ja/nein
          <BoolInput label = "Bezug von Biogas" name="biogas" value = biogas.as_deref()== Some("yes") comment = None />

          // Strombedarf gesamt
          <NumberInput
            label = "Strombedarf gesamt"
            name="strombedarf"
            placeholder="Gesamtstrombedarf"
            value = strombedarf
            unit = "kWh/a"
          />

          // Eigenstromerzeugung
          <NumberInput
            label = "Eigenstromerzeugung"
            name="eigenstrom"
            placeholder="Eigenstrom"
            value = eigenstrom
            unit = "kWh/a"
          />

          // Emissionsfaktor Strom-Mix
          <NumberInput
            label = "Emissionsfaktor Strommix (Versorger)"
            name="ef_strommix"
            //defaultValue = 485
            placeholder="485"
            value = ef_strommix
            unit = "g CO₂/kWh"
          />

        </div>
      </div>

      // Klärschlammbehandlung
      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">Klärschlammbehandlung</h3>

        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          <BoolInput
            name="schlammtaschen"
            label = "Offene Schlammtaschen"
            value = schlammtaschen.as_deref() == Some("yes")
            comment = Some("ja/nein")
          />

          <BoolInput
            name="schlammstapel"
            label = "Offene Schlammstapelbehälter"
            value = schlammstapel.as_deref() == Some("yes")
            comment = Some("ja/nein")
          />

          <NumberInput
            label = "Kläraschlamm zur Entsorgung"
            unit = "t"
            name = "klaerschlamm_enstorgung"
            placeholder = "Masse entwässert"
            value = klaerschlamm_enstorgung
          />

          <NumberInput
            label = "Transportdistanz"
            unit = "km"
            name="klaerschlamm_transport"
            placeholder="Entfernung"
            value = klaerschlamm_transport
          />

        </div>
      </div>

      // Betriebsstoffe
      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">Eingesetzte Betriebsstoffe</h3>
        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          // Eisen(III)Chlorid
          <NumberInput
            label = "Eisen(III)-chlorid-Lösung"
            unit = "kg"
            name = "betriebsstoffe_fe3"
            placeholder = "kg Lösung"
            value = betriebsstoffe_fe3
          />

          // Eisen(III)Chlorid
          <NumberInput
            label = "Eisenchloridsulfat-Lösung"
            unit = "kg"
            name = "betriebsstoffe_feso4"
            placeholder = "kg Lösung"
            value = betriebsstoffe_feso4
          />

          // Kalkhydrat
          <NumberInput
            label = "Kalkhydrat"
            unit = "kg"
            name = "betriebsstoffe_kalk"
            placeholder = "kg Branntkalk"
            value = betriebsstoffe_kalk
          />

          // Polymere
          <NumberInput
            label = "Synthetische Polymere"
            placeholder = "kg Polymere"
            name = "betriebsstoffe_poly"
            unit = "kg"
            value = betriebsstoffe_poly
          />

        </div>
      </div>

      // Szenario
      <div class="border-b border-gray-900/10 pb-12">
        <h3 class="text-lg font-semibold leading-7 text-gray-900">Schätzung des N<sub>2</sub>O-Emissionsfaktors</h3>
        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

          <div>
            <label for="n2o_szenario" class="block text-sm font-bold leading-6 text-gray-900">Szenario</label>
            <select name="n2o_szenario" class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6">
              <option value="0" selected = (n2o_szenario == "0")>Extrapoliert nach Parravicini et al. 2016</option>
              <option value="1" selected = (n2o_szenario == "1")>Optimistisch</option>
              <option value="2" selected = (n2o_szenario == "2")>Pessimistisch</option>
              <option value="3" selected = (n2o_szenario == "3")>Nach IPCC 2019</option>
            </select>
          </div>

        </div>
      </div>
      <div class="mt-6 flex items-center justify-end gap-x-6">
        <input
          type="submit"
          class="rounded-md bg-indigo-600 px-3 py-2 text-lg font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          value = "Start" />
      </div>

    </form>
    }
}

#[component]
fn NumberInput(
    label: &'static str,
    unit: &'static str,
    placeholder: &'static str,
    name: &'static str,
    value: String,
) -> impl IntoView {
    let id = format!("form-input-{name}");

    view! {
      <div>
        <label for={ id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id
            type="text"
            name = { name }
            maxlength = 8
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            value = { value }
          />
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
            <span class="text-gray-500 sm:text-sm">{ unit }</span>
          </div>
        </div>
      </div>
    }
}

#[component]
fn BoolInput(
    label: &'static str,
    name: &'static str,
    value: bool,
    comment: Option<&'static str>,
) -> impl IntoView {
    let id = format!("form-input-{name}");

    view! {
      <div class="relative flex items-start">
        <div class="flex h-6 items-center">
          <input
            id
            name
            type="checkbox"
            class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
            // TODO: aria-describedby
            checked = value
          />
        </div>
        <div class="ml-3 text-sm leading-6">
          <label for={ id } class="font-bold text-gray-900">{ label }</label>
          <p class="text-gray-500">{ comment }</p>
        </div>
      </div>
    }
}

#[component]
fn TextInput(
    label: &'static str,
    name: &'static str,
    placeholder: &'static str,
    value: String,
) -> impl IntoView {
    let id = format!("form-input-{name}");

    view! {
      <div>
        <label for={ id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id
            type="text"
            name = { name }
            maxlength = 8
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            value = { value }
          />
        </div>
      </div>
    }
}
