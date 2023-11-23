use crate::{InputData, OperatingMaterials};

use super::Import;

const V1_OPERATING_MATERIALS_DIVISOR: f64 = 1_000.0;

pub fn from_v1(mut data: Import) -> Import {
    let Import {
        input: InputData {
            operating_materials,
            ..
        },
        ..
    } = data;
    let OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    let map_value = |v| v / V1_OPERATING_MATERIALS_DIVISOR;
    let fecl3 = fecl3.map(map_value);
    let feclso4 = feclso4.map(map_value);
    let caoh2 = caoh2.map(map_value);
    let synthetic_polymers = synthetic_polymers.map(map_value);

    data.input.operating_materials = OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    };

    data
}
