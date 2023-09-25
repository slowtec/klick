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

    // Allgemeine Infos zur Kläranlage
    view! {
    <form action="/submit" method="post" class="ui form">
      <h3 class="ui dividing header">Angaben zur Kläranlage</h3>
        <div class="fields">
          // Name der Kläranlage
          <div class="field">
            <label>Name oder Ort</label>
            <input type="text" name="name" id = "name" placeholder="Name der Kläranlage" value = name />
          </div>
          // EW-Werte
          <div class="field">
            <label>Ausbaugröße</label>
            <div class="ui right labeled input">
              <input type="text" name="ew" id = "ew" maxlength="8" placeholder="Ausbaugröße [EW]" value = ew />
              <div class="ui basic label">
                EW
              </div>
            </div>
          </div>
          // Abwassermenge
          <div class="field">
            <label>Abwassermenge</label>
            <div class="ui right labeled input">
              <input type="text" name="flow" id = "flow" placeholder="Abwassermenge" value = flow />
              <div class="ui basic label">
                m<sup>3</sup>/a
              </div>
            </div>
          </div>
        </div>

      // Zulauf-Parameter
      <h3 class="ui dividing header">Zulauf-Parameter (Jahresmittelwerte)</h3>
      <div class="fields">
        // CSB
        <div class="field">
          <label>Chemischer Sauerstoffbedarf</label>
          <div class="ui right labeled input">
            <input type="text" name="csb_zu" maxlength="8" placeholder="CSB" value = csb_zu />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
        // TKN
        <div class="field">
          <label>Gesamtstickstoff</label>
          <div class="ui right labeled input">
            <input type="text" name="tkn_zu" maxlength="8" placeholder="TKN" value = tkn_zu />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
        // P
        <div class="field">
          <label>Phosphor</label>
          <div class="ui right labeled input">
            <input type="text" name="p_zu" maxlength="8" placeholder="P" value = p_zu />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
      </div>

      // Ablauf-Parameter
      <h3 class="ui dividing header">Ablauf-Parameter (Jahresmittelwerte)</h3>
      <div class="fields">
        // CSB
        <div class="field">
          <label>Chemischer Sauerstoffbedarf</label>
          <div class="ui right labeled input">
            <input type="text" name="csb_ab" maxlength="8" placeholder="CSB" value = csb_ab />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
        // TKN
        <div class="field">
          <label>Gesamtstickstoff</label>
          <div class="ui right labeled input">
            <input type="text" name="tkn_ab" maxlength="8" placeholder="TKN" value = tkn_ab />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
        // P
        <div class="field">
          <label>Phosphor</label>
          <div class="ui right labeled input">
            <input type="text" name="p_ab" maxlength="8" placeholder="P" value = p_ab />
            <div class="ui basic label">
              mg/L
            </div>
          </div>
        </div>
      </div>

      // Energiebedarf
      <h3 class="ui dividing header">Energiebedarf</h3>
      <div class="fields">
        // Klärgas erzeugt
        <div class="field">
          <label>Erzeugtes Klärgas</label>
          <div class="ui right labeled input">
            <input type="text" name="klaergas" maxlength="8" placeholder="Klärgas" value = klaergas />
            <div class="ui basic label">
              m<sup>3</sup>/a
            </div>
          </div>
        </div>
        <div class="field">
          <label>Methangehalt</label>
          <div class="ui right labeled input">
            <input type="text" name="methangehalt" maxlength="8" placeholder="65" value = methangehalt />
            <div class="ui basic label">
              %
            </div>
          </div>
        </div>
      </div>

      // Erdgas zugekauft
      <div class="fields">
        <div class="field">
          <label>Gasbezug (Versorger)</label>
          <div class="ui right labeled input">
            <input type="text" name="gas_zusatz" maxlength="8" placeholder="Gasbezug" value = gas_zusatz />
            <div class="ui basic label">
              kWh/a
            </div>
          </div>
        </div>
        // Biogas ja/nein
        <div class="ui checkbox">
          <input type="checkbox" name="biogas" id="biogas" checked = (biogas.as_deref()== Some("yes")) value = biogas />
          <label>Bezug von Biogas</label>
        </div>
      </div>
      <div class="fields">
        // Strombedarf gesamt
        <div class="field">
          <label>Strombedarf gesamt</label>
          <div class="ui right labeled input">
            <input type="text" name="strombedarf" maxlength="8" placeholder="Gesamtstrombedarf" value = strombedarf />
            <div class="ui basic label">
              kWh/a
            </div>
          </div>
        </div>
        // Eigenstromerzeugung
        <div class="field">
          <label>Eigenstromerzeugung</label>
          <div class="ui right labeled input">
            <input type="text" name="eigenstrom" maxlength="8" placeholder="Eigenstrom" value = eigenstrom />
            <div class="ui basic label">
              kWh/a
            </div>
          </div>
        </div>
        // Emissionsfaktor Strom-Mix
        <div class="field">
          <label>Emissionsfaktor Strommix (Versorger)</label>
          <div class="ui right labeled input">
            <input type="text" name="ef_strommix" maxlength="8" defaultValue = 485 placeholder="485" value = ef_strommix />
            <div class="ui basic label">
              g CO<sub>2</sub>/kWh
            </div>
          </div>
        </div>
      </div>

      // Klärschlammbehandlung
      <h3 class="ui dividing header">Klärschlammbehandlung</h3>
      <div class="fields">
        // Offene Schlammtaschen ja/nein
        <div class="ui checkbox">
          <input type="checkbox" name="schlammtaschen" id="schlammtaschen" checked = (schlammtaschen.as_deref() == Some("yes")) />
          <label>Offene Schlammtaschen</label>
        </div>
      </div>
      <div class="fields">
        // Offene Schlammstapelbehälter ja/nein
        <div class="ui checkbox">
          <input type="checkbox" name="schlammstapel" id="schlammstapel" checked = (schlammstapel.as_deref() == Some("yes")) />
          <label>Offene Schlammstapelbehälter</label>
        </div>
      </div>
      <div class="fields">
        <div class="field">
          <label>Kläraschlamm zur Entsorgung</label>
          <div class="ui right labeled input">
            <input type="text" name="klaerschlamm_enstorgung" maxlength="8" placeholder="Masse entwässert" value = klaerschlamm_enstorgung />
            <div class="ui basic label">
              t
            </div>
          </div>
        </div>
        <div class="field">
          <label>Transportdistanz</label>
          <div class="ui right labeled input">
            <input type="text" name="klaerschlamm_transport" maxlength="8" placeholder="Entfernung" value = klaerschlamm_transport />
            <div class="ui basic label">
              km
            </div>
          </div>
        </div>
      </div>

      // Betriebsstoffe
      <h3 class="ui dividing header">Eingesetzte Betriebsstoffe</h3>
      <div class="fields">

        // Eisen(III)Chlorid
        <div class="field">
          <label>Eisen(III)-chlorid-Lösung</label>
          <div class="ui right labeled input">
            <input type="text" name="betriebsstoffe_fe3" maxlength="8" placeholder="kg Lösung" value = betriebsstoffe_fe3 />
            <div class="ui basic label">
              kg
            </div>
          </div>
        </div>
        // Eisen(III)Chlorid
        <div class="field">
          <label>Eisenchloridsulfat-Lösung</label>
          <div class="ui right labeled input">
            <input type="text" name="betriebsstoffe_feso4" maxlength="8" placeholder="kg Lösung" value = betriebsstoffe_feso4 />
            <div class="ui basic label">
              kg
            </div>
          </div>
        </div>

        // Kalkhydrat
        <div class="field">
          <label>Kalkhydrat</label>
          <div class="ui right labeled input">
            <input type="text" name="betriebsstoffe_kalk" maxlength="8" placeholder="kg Branntkalk" value = betriebsstoffe_kalk />
            <div class="ui basic label">
              kg
            </div>
          </div>
        </div>

        // Polymere
        <div class="field">
          <label>Synthetische Polymere</label>
          <div class="ui right labeled input">
            <input type="text" name="betriebsstoffe_poly" maxlength="8" placeholder="kg Polymere" value = betriebsstoffe_poly  />
            <div class="ui basic label">
              kg
            </div>
          </div>
        </div>
      </div>

      // Szenario
      <h3 class="ui dividing header">Schätzung des N<sub>2</sub>O-Emissionsfaktors</h3>
      <div class="fields">
        <div class="field">
          <select class="ui dropdown" name="n2o_szenario">
            <option value="0" selected = (n2o_szenario == "0")>Extrapoliert nach Parravicini et al. 2016</option>
            <option value="1" selected = (n2o_szenario == "1")>Optimistisch</option>
            <option value="2" selected = (n2o_szenario == "2")>Pessimistisch</option>
            <option value="3" selected = (n2o_szenario == "3")>Nach IPCC 2019</option>
          </select>
        </div>
      </div>
      <input class="ui button primary" type="submit" value="Start" />
    </form>
    }
}
