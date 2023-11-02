use klick_application as app;

use crate::{InputData, N2OSzenario, OutputData, ValueId};

impl From<ValueId> for app::ValueId {
    fn from(from: ValueId) -> Self {
        use app::ValueId as A;
        use ValueId as F;

        match from {
            F::Name => A::Name,
            F::Ew => A::Ew,
            F::Flow => A::Flow,
            F::CsbZu => A::CsbZu,
            F::TknZu => A::TknZu,
            F::PZu => A::PZu,
            F::CsbAb => A::CsbAb,
            F::TknAb => A::TknAb,
            F::PAb => A::PAb,
            F::Klaergas => A::Klaergas,
            F::Methangehalt => A::Methangehalt,
            F::GasZusatz => A::GasZusatz,
            F::Biogas => A::Biogas,
            F::Strombedarf => A::Strombedarf,
            F::Eigenstrom => A::Eigenstrom,
            F::EfStrommix => A::EfStrommix,
            F::Schlammtaschen => A::Schlammtaschen,
            F::Schlammstapel => A::Schlammstapel,
            F::KlaerschlammEnstorgung => A::KlaerschlammEnstorgung,
            F::KlaerschlammTransport => A::KlaerschlammTransport,
            F::BetriebsstoffeFe3 => A::BetriebsstoffeFe3,
            F::BetriebsstoffeFeso4 => A::BetriebsstoffeFeso4,
            F::BetriebsstoffeKalk => A::BetriebsstoffeKalk,
            F::BetriebsstoffePoly => A::BetriebsstoffePoly,
            F::N2oSzenario => A::N2oSzenario,
        }
    }
}

impl From<app::ValueId> for ValueId {
    fn from(from: app::ValueId) -> Self {
        use app::ValueId as A;
        use ValueId as F;

        match from {
            A::Name => F::Name,
            A::Ew => F::Ew,
            A::Flow => F::Flow,
            A::CsbZu => F::CsbZu,
            A::TknZu => F::TknZu,
            A::PZu => F::PZu,
            A::CsbAb => F::CsbAb,
            A::TknAb => F::TknAb,
            A::PAb => F::PAb,
            A::Klaergas => F::Klaergas,
            A::Methangehalt => F::Methangehalt,
            A::GasZusatz => F::GasZusatz,
            A::Biogas => F::Biogas,
            A::Strombedarf => F::Strombedarf,
            A::Eigenstrom => F::Eigenstrom,
            A::EfStrommix => F::EfStrommix,
            A::Schlammtaschen => F::Schlammtaschen,
            A::Schlammstapel => F::Schlammstapel,
            A::KlaerschlammEnstorgung => F::KlaerschlammEnstorgung,
            A::KlaerschlammTransport => F::KlaerschlammTransport,
            A::BetriebsstoffeFe3 => F::BetriebsstoffeFe3,
            A::BetriebsstoffeFeso4 => F::BetriebsstoffeFeso4,
            A::BetriebsstoffeKalk => F::BetriebsstoffeKalk,
            A::BetriebsstoffePoly => F::BetriebsstoffePoly,
            A::N2oSzenario => F::N2oSzenario,
        }
    }
}

impl From<N2OSzenario> for app::N2OSzenario {
    fn from(from: N2OSzenario) -> Self {
        use app::N2OSzenario as A;
        use N2OSzenario as F;

        match from {
            F::ExtrapolatedParravicini => A::ExtrapolatedParravicini,
            F::Optimistic => A::Optimistic,
            F::Pesimistic => A::Pesimistic,
            F::Ipcc2019 => A::Ipcc2019,
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
        }
    }
}
