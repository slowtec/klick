use anyhow::bail;

use klick_domain::{self as domain, units::*};

use crate::*;

// -----   ----- //
//    Project    //
// -----   ----- //

impl From<ProjectId> for domain::ProjectId {
    fn from(from: ProjectId) -> Self {
        Self::from_uuid(from.0)
    }
}

impl From<domain::ProjectId> for ProjectId {
    fn from(from: domain::ProjectId) -> Self {
        Self(from.to_uuid())
    }
}

impl From<SavedProject> for domain::Project<JsonFormData> {
    fn from(from: SavedProject) -> Self {
        let SavedProject {
            id,
            created_at,
            modified_at,
            data,
        } = from;
        let id = domain::ProjectId::from(id);

        Self {
            id,
            created_at,
            modified_at,
            data,
        }
    }
}

impl From<domain::Project<JsonFormData>> for SavedProject {
    fn from(from: domain::Project<JsonFormData>) -> Self {
        let domain::Project {
            id,
            created_at,
            modified_at,
            data,
        } = from;

        let id = id.into();

        Self {
            id,
            created_at,
            modified_at,
            data,
        }
    }
}

impl From<domain::Project<JsonFormData>> for Project {
    fn from(from: domain::Project<JsonFormData>) -> Self {
        Self::Saved(from.into())
    }
}

// -----   ----- //
//    Values     //
// -----   ----- //

impl TryFrom<CH4ChpEmissionsSensitivity>
    for (
        Option<domain::units::Ch4ChpEmissionFactorCalcMethod>,
        Option<Factor>,
    )
{
    type Error = anyhow::Error;

    fn try_from(from: CH4ChpEmissionsSensitivity) -> Result<Self, Self::Error> {
        use crate::CH4ChpEmissionFactorCalcMethod as M;
        use domain::units::Ch4ChpEmissionFactorCalcMethod as D;

        let method = from.calculation_method.map(|method| match method {
            M::MicroGasTurbines => D::MicroGasTurbines,
            M::GasolineEngine => D::GasolineEngine,
            M::JetEngine => D::JetEngine,
            M::CustomFactor => D::Custom,
        });

        let custom_factor = from
            .custom_emission_factor
            .map(|f| Percent::new(f).convert_to());

        if method == Some(D::Custom) && custom_factor.is_none() {
            bail!("custom CH4 CHP emission factor is missing");
        }

        Ok((method, custom_factor))
    }
}

impl From<domain::units::N2oEmissionFactorCalcMethod> for crate::N2oEmissionFactorCalcMethod {
    fn from(from: domain::units::N2oEmissionFactorCalcMethod) -> Self {
        use domain::units::N2oEmissionFactorCalcMethod as FROM;
        match from {
            FROM::TuWien2016 => Self::TuWien2016,
            FROM::Optimistic => Self::Optimistic,
            FROM::Pesimistic => Self::Pesimistic,
            FROM::Ipcc2019 => Self::Ipcc2019,
            FROM::Custom => Self::CustomFactor,
        }
    }
}

impl From<domain::units::Ch4ChpEmissionFactorCalcMethod> for crate::CH4ChpEmissionFactorCalcMethod {
    fn from(from: domain::units::Ch4ChpEmissionFactorCalcMethod) -> Self {
        use domain::units::Ch4ChpEmissionFactorCalcMethod as FROM;
        match from {
            FROM::MicroGasTurbines => Self::MicroGasTurbines,
            FROM::GasolineEngine => Self::GasolineEngine,
            FROM::JetEngine => Self::JetEngine,
            FROM::Custom => Self::CustomFactor,
        }
    }
}

impl From<crate::N2oEmissionFactorCalcMethod> for domain::units::N2oEmissionFactorCalcMethod {
    fn from(from: crate::N2oEmissionFactorCalcMethod) -> Self {
        use crate::N2oEmissionFactorCalcMethod as FROM;
        match from {
            FROM::TuWien2016 => Self::TuWien2016,
            FROM::Optimistic => Self::Optimistic,
            FROM::Pesimistic => Self::Pesimistic,
            FROM::Ipcc2019 => Self::Ipcc2019,
            FROM::CustomFactor => Self::Custom,
        }
    }
}

impl From<crate::CH4ChpEmissionFactorCalcMethod> for domain::units::Ch4ChpEmissionFactorCalcMethod {
    fn from(from: crate::CH4ChpEmissionFactorCalcMethod) -> Self {
        use crate::CH4ChpEmissionFactorCalcMethod as FROM;
        match from {
            FROM::MicroGasTurbines => Self::MicroGasTurbines,
            FROM::GasolineEngine => Self::GasolineEngine,
            FROM::JetEngine => Self::JetEngine,
            FROM::CustomFactor => Self::Custom,
        }
    }
}
