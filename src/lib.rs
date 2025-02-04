use wasm_bindgen::prelude::*;
use h3o::{CellIndex, LatLng, DirectedEdgeIndex};
use h3o::{
 geom::{ContainmentMode, TilerBuilder},
 Resolution};

use js_sys::Array;
use geo::{Coord, LineString, Polygon};
use serde_wasm_bindgen::from_value;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isValidCell(hex_index: &str) -> bool {
    if let Ok(index) = u64::from_str_radix(hex_index, 16) {
        if let Ok(_) = CellIndex::try_from(index) {
            return true;
        }
    }
    false
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getResolution(h3Index: &str) -> Result<u8, JsValue> {
    let h3_index = u64::from_str_radix(h3Index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let resolution_value: u8 = cell_index.resolution().into();
    Ok(resolution_value)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isResClassIII(hex_index: &str) -> Result<bool, JsValue> {
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let resolution: Resolution = cell_index.resolution();
    Ok(resolution.is_class3())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getBaseCellNumber(h3Index: &str) -> Result<u32, JsValue> {
    let h3_index = u64::from_str_radix(h3Index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let base_cell_u8: u8 = cell_index.base_cell().into();
    let base_cell_number = base_cell_u8 as u32;

    Ok(base_cell_number)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isPentagon(hex_index: &str) -> Result<bool, JsValue> {
    // Attempt to parse the hexadecimal string to a u64
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let pentagon = cell_index.is_pentagon();

    Ok(pentagon)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellArea(h3Index: &str, unit: &str) -> Result<f64, JsValue> {
    // Attempt to parse the hexadecimal string to a u64
    let h3_index_u64 = u64::from_str_radix(h3Index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    // Use the parsed index to create a CellIndex and proceed as before
    let cell_index = CellIndex::try_from(h3_index_u64)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let area = match unit {
        "km2" => cell_index.area_km2(),
        "m2" => cell_index.area_m2(),
        "rads2"  => cell_index.area_rads2(),
        _ => return Err(JsValue::from_str("Invalid unit specified. Use 'km2','m2' or 'rads2'.")),
    };

    Ok(area)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn edgeLength(edge_index: &str, unit: &str) -> Result<f64, JsValue> {
    // Attempt to parse the hexadecimal string to a u64
    let index = u64::from_str_radix(edge_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    // Use the parsed index to create a CellIndex and proceed as before
    let directed_edge_index = DirectedEdgeIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;


    let edge_length = match unit {
        "km" => directed_edge_index.length_km(),
        "m" => directed_edge_index.length_m(),
        "rads"  => directed_edge_index.length_rads(),
        _ => return Err(JsValue::from_str("Invalid unit specified. Use 'km','m' or 'rads'.")),
    };

    Ok(edge_length)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToLatLng(h3_index: &str) -> Result<JsValue, JsValue> {
    let index = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let latlng = LatLng::from(cell_index);

    let lat = latlng.lat();
    let lng = latlng.lng();

    let latlng_array = Array::new();
    latlng_array.push(&JsValue::from_f64(lat));
    latlng_array.push(&JsValue::from_f64(lng));

    Ok(latlng_array.into())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToLngLat(h3_index: &str) -> Result<JsValue, JsValue> {
    let index = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let latlng = LatLng::from(cell_index);

    let lng = latlng.lng();
    let lat = latlng.lat();

    let lnglat_array = Array::new();
    lnglat_array.push(&JsValue::from_f64(lng));
    lnglat_array.push(&JsValue::from_f64(lat));

    Ok(lnglat_array.into())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToBoundary(h3_index: &str) -> Result<JsValue, JsValue> {
    let index = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let boundary = cell_index.boundary();

    // Collect all coordinates into a Vec<JsValue>
    let boundary_vec: Vec<JsValue> = boundary
        .iter()
        .map(|latlng| {
            let coord_array = js_sys::Array::of2(
                &JsValue::from_f64(latlng.lat()),
                &JsValue::from_f64(latlng.lng()),
            );
            coord_array.into()
        })
        .collect();

    // Convert the Vec<JsValue> into a js_sys::Array
    let boundary_array = js_sys::Array::from(&boundary_vec.into());

    Ok(boundary_array.into())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToChildren(h3_index: &str, res: u8) -> Result<JsValue, JsValue> {
    let index = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let resolution = Resolution::try_from(res)
        .map_err(|e| JsValue::from_str(&format!("Invalid resolution: {}", e)))?;

    let children = cell_index.children(resolution);

    let children_vec: Vec<_> = children
        .into_iter()
        .map(|child| JsValue::from_str(&child.to_string()))
        .collect();

    let js_array = js_sys::Array::from(&children_vec.into());

    Ok(js_array.into())
}


#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToParent(h3_index: &str, res: u8) -> Result<JsValue, JsValue> {
    // Try converting the input h3Index string to a CellIndex
    let h3_index_u64 = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index_u64)
        .map_err(|e| JsValue::from_str(&format!("Invalid H3 index: {}", e)))?;

    let resolution = Resolution::try_from(res)
        .map_err(|e| JsValue::from_str(&format!("Invalid resolution: {}", e)))?;

    let parent = cell_index
        .parent(resolution)
        .ok_or_else(|| JsValue::from_str("Failed to find parent at given resolution"))?;

    Ok(JsValue::from_str(&parent.to_string()))
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToCenterChild(h3Index: &str, res: u8) -> Result<JsValue, JsValue> {
    let h3_index_u64 = u64::from_str_radix(h3Index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index_u64)
        .map_err(|e| JsValue::from_str(&format!("Invalid H3 index: {}", e)))?;

    let resolution = Resolution::try_from(res)
        .map_err(|e| JsValue::from_str(&format!("Invalid resolution: {}", e)))?;

    let center_child = cell_index
        .center_child(resolution)
        .ok_or_else(|| JsValue::from_str("Failed to find center child at given resolution"))?;

    Ok(JsValue::from_str(&center_child.to_string()))
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn latLngToCell(lat: f64, lng: f64, resolution: u8) -> Result<JsValue, JsValue> {
    let ll = LatLng::new(lat, lng).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let resolution = Resolution::try_from(resolution).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let cellIndex = ll.to_cell(resolution);

    Ok(JsValue::from_str(&cellIndex.to_string()))
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToLocalIj(h3_origin_index: &str, h3_index: &str) -> Result<JsValue, JsValue> {

    let h3_origin_index_u64 = u64::from_str_radix(h3_origin_index, 16)
            .map_err(|e| JsValue::from_str(&format!("Invalid origin hex string: {}", e)))?;

    let cell_origin_index = CellIndex::try_from(h3_origin_index_u64)
            .map_err(|e| JsValue::from_str(&format!("Invalid origin H3 index: {}", e)))?;


    let h3_index_u64 = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index_u64)
        .map_err(|e| JsValue::from_str(&format!("Invalid H3 index: {}", e)))?;


    let local_ij = cell_index.to_local_ij(cell_origin_index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let coords = local_ij.coord;
    let i = coords.i;
    let j = coords.j;

    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &JsValue::from_str("i"), &JsValue::from(i)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("j"), &JsValue::from(j)).unwrap();

    Ok(obj.into())
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn polygonToCells(polygon_js: &JsValue, resolution_js: u8) -> Result<JsValue, JsValue> {
    let resolution = Resolution::try_from(resolution_js)
        .map_err(|_| JsValue::from_str("Invalid resolution"))?;

    let polygon_coords: Vec<Vec<f64>> = from_value(polygon_js.clone())
        .map_err(|_| JsValue::from_str("Failed to parse polygon coordinates"))?;

    let exterior_coords: Vec<Coord<f64>> = polygon_coords.iter()
        .map(|pair| Coord { x: pair[1], y: pair[0] })
        .collect();

    let polygon = Polygon::new(LineString(exterior_coords), vec![]);

    let mut tiler = TilerBuilder::new(resolution)
        .containment_mode(ContainmentMode::ContainsBoundary)
        .build();

    tiler.add(polygon)
        .map_err(|_| JsValue::from_str("Failed to add polygon to tiler"))?;

    let cells_iter = tiler.into_coverage();

    let cells_array = js_sys::Array::new();
    for cell_index in cells_iter {
        cells_array.push(&JsValue::from_str(&cell_index.to_string()));
    }

    Ok(JsValue::from(cells_array))
}
