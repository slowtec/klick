use klick_application as app;

use crate::{InputData, N2OSzenario, OutputData};

impl From<N2OSzenario> for app::N2OSzenario {
    fn from(from: N2OSzenario) -> Self {
        use app::N2OSzenario as A;
        use N2OSzenario as F;

        match from {
            F::ExtrapolatedParravicini => A::ExtrapolatedParravicini,
            F::Optimistic => A::Optimistic,
            F::Pesimistic => A::Pesimistic,
            F::Ipcc2019 => A::Ipcc2019,
            F::Custom => A::Custom,
        }
    }
}

impl From<app::N2OSzenario> for N2OSzenario {
    fn from(from: app::N2OSzenario) -> Self {
        use app::N2OSzenario as A;
        use N2OSzenario as F;

        match from {
            A::ExtrapolatedParravicini => F::ExtrapolatedParravicini,
            A::Optimistic => F::Optimistic,
            A::Pesimistic => F::Pesimistic,
            A::Ipcc2019 => F::Ipcc2019,
            A::Custom => F::Custom,
        }
    }
}

impl From<InputData> for app::InputData {
    fn from(from: InputData) -> Self {
        let InputData {
            ew,
            abwasser,
            n_ges_zu,
            csb_ab,
            n_ges_ab,
            klaergas_gesamt,
            methangehalt,
            strombedarf,
            energie_eigen,
            ef_co2_strommix,
            schlammtaschen,
            schlammstapel,
            klaerschlamm_transport_km,
            klaerschlamm_entsorgung_m,
            betriebsstoffe_fe3,
            betriebsstoffe_feso4,
            betriebsstoffe_kalk,
            betriebsstoffe_poly,
            n2o_szenario,
            custom_n2o_szenario_value,
        } = from;

        let n2o_szenario = n2o_szenario.into();

        Self {
            ew,
            abwasser,
            n_ges_zu,
            csb_ab,
            n_ges_ab,
            klaergas_gesamt,
            methangehalt,
            strombedarf,
            energie_eigen,
            ef_co2_strommix,
            schlammtaschen,
            schlammstapel,
            klaerschlamm_transport_km,
            klaerschlamm_entsorgung_m,
            betriebsstoffe_fe3,
            betriebsstoffe_feso4,
            betriebsstoffe_kalk,
            betriebsstoffe_poly,
            n2o_szenario,
            custom_n2o_szenario_value,
        }
    }
}

impl From<app::InputData> for InputData {
    fn from(from: app::InputData) -> Self {
        let app::InputData {
            ew,
            abwasser,
            n_ges_zu,
            csb_ab,
            n_ges_ab,
            klaergas_gesamt,
            methangehalt,
            strombedarf,
            energie_eigen,
            ef_co2_strommix,
            schlammtaschen,
            schlammstapel,
            klaerschlamm_transport_km,
            klaerschlamm_entsorgung_m,
            betriebsstoffe_fe3,
            betriebsstoffe_feso4,
            betriebsstoffe_kalk,
            betriebsstoffe_poly,
            n2o_szenario,
            custom_n2o_szenario_value,
        } = from;

        let n2o_szenario = n2o_szenario.into();

        Self {
            ew,
            abwasser,
            n_ges_zu,
            csb_ab,
            n_ges_ab,
            klaergas_gesamt,
            methangehalt,
            strombedarf,
            energie_eigen,
            ef_co2_strommix,
            schlammtaschen,
            schlammstapel,
            klaerschlamm_transport_km,
            klaerschlamm_entsorgung_m,
            betriebsstoffe_fe3,
            betriebsstoffe_feso4,
            betriebsstoffe_kalk,
            betriebsstoffe_poly,
            n2o_szenario,
            custom_n2o_szenario_value,
        }
    }
}

impl From<OutputData> for app::OutputData {
    fn from(from: OutputData) -> Self {
        let OutputData {
            co2eq_n2o_anlage,
            co2eq_n2o_gewaesser,
            co2eq_ch4_klaerprozes,
            co2eq_ch4_schlammstapel,
            co2eq_ch4_schlammtasche,
            co2eq_ch4_gewaesser,
            co2eq_ch4_bhkw,
            co2eq_betriebsstoffe_fe3,
            co2eq_betriebsstoffe_feso4,
            co2eq_betriebsstoffe_kalk,
            co2eq_betriebsstoffe_poly,
            co2eq_strommix,
            co2eq_betriebsstoffe,
            co2eq_klaerschlamm_transport,
            direkte_emissionen_co2_eq,
            indirekte_emissionen_co2_eq,
            weitere_indirekte_emissionen_co2_eq,
            emissionen_co2_eq,
            ef_n2o_anlage,
        } = from;

        Self {
            co2eq_n2o_anlage,
            co2eq_n2o_gewaesser,
            co2eq_ch4_klaerprozes,
            co2eq_ch4_schlammstapel,
            co2eq_ch4_schlammtasche,
            co2eq_ch4_gewaesser,
            co2eq_ch4_bhkw,
            co2eq_betriebsstoffe_fe3,
            co2eq_betriebsstoffe_feso4,
            co2eq_betriebsstoffe_kalk,
            co2eq_betriebsstoffe_poly,
            co2eq_strommix,
            co2eq_betriebsstoffe,
            co2eq_klaerschlamm_transport,
            direkte_emissionen_co2_eq,
            indirekte_emissionen_co2_eq,
            weitere_indirekte_emissionen_co2_eq,
            emissionen_co2_eq,
            ef_n2o_anlage,
        }
    }
}
