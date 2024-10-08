// TODO: move to value_metadata

use fluent_templates::Loader;

use klick_domain::{self as domain, InputValueId, OutputValueId, ValueId};

use crate::{Lng, LOCALES};

pub trait ValueLabel {
    fn label(&self, lng: Lng) -> String;
    fn label_latex(&self, lng: Lng) -> String {
        self.label(lng)
    }
}

impl ValueLabel for InputValueId {
    fn label(&self, lng: Lng) -> String {
        LOCALES.lookup(&lng.id(), self.as_ref())
    }

    fn label_latex(&self, lng: Lng) -> String {
        match self {
            Self::SensitivityN2OCalculationMethod => "$N_2O$ Berechnungsmethode",
            Self::SensitivityN2OCustomFactor => "$N_2O$-EF Benutzerdefiniert",
            Self::SensitivityN2OSideStreamFactor => "$N_2O$-EF Prozesswasser",
            Self::SensitivityCH4ChpCustomFactor => "BHKW $CH_4$-EF benutzerdefiniert",
            Self::SensitivityCO2FossilCustomFactor => "$CO_2$-EF (fossil)",
            Self::SensitivitySludgeBagsCustomFactor => "$CH_4$-EF Schlammtaschen",
            Self::SensitivitySludgeStorageCustomFactor => "$CH_4$-EF Schlammlagerung",
            _ => {
                return self.label(lng);
            }
        }
        .to_string()
    }
}

impl ValueLabel for domain::units::N2oEmissionFactorCalcMethod {
    fn label(&self, lng: Lng) -> String {
        LOCALES.lookup(&lng.id(), self.as_ref())
    }
}

impl ValueLabel for domain::units::Ch4ChpEmissionFactorCalcMethod {
    fn label(&self, lng: Lng) -> String {
        LOCALES.lookup(&lng.id(), self.as_ref())
    }
}

impl ValueLabel for OutputValueId {
    fn label(&self, lng: Lng) -> String {
        LOCALES.lookup(&lng.id(), self.as_ref())
    }
}

impl ValueLabel for ValueId {
    fn label(&self, lng: Lng) -> String {
        match self {
            ValueId::Custom(name) => name.clone(),
            ValueId::In(id) => id.label(lng),
            ValueId::Out(id) => id.label(lng),
        }
    }
}
