use wasm_bindgen::prelude::*;
use h3o::{CellIndex, LatLng, Resolution, DirectedEdgeIndex};
use h3o::geom::{PolyfillConfig, Polygon, ToCells};
use js_sys::Array;
use geo::{Coord, LineString, Polygon as GeoPolygon};
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
pub fn getResolution(hex_index: &str) -> Result<u8, JsValue> {
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Use the From trait implementation to convert Resolution to u8
    let resolution_value: u8 = cell_index.resolution().into();
    Ok(resolution_value)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getBaseCellNumber(hex_index: &str) -> Result<u32, JsValue> {
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Convert BaseCell to u8 using the From trait
    let base_cell_u8: u8 = cell_index.base_cell().into();

    // Then convert or cast u8 to u32 to match the function's expected return type
    let base_cell_number = base_cell_u8 as u32;

    Ok(base_cell_number)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn isPentagon(hex_index: &str) -> Result<bool, JsValue> {
    // Attempt to parse the hexadecimal string to a u64
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    // Use the parsed index to create a CellIndex
    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Use the is_pentagon method to determine if the cell is a pentagon
    let pentagon = cell_index.is_pentagon();

    Ok(pentagon)
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellArea(hex_index: &str, unit: &str) -> Result<f64, JsValue> {
    // Attempt to parse the hexadecimal string to a u64
    let index = u64::from_str_radix(hex_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    // Use the parsed index to create a CellIndex and proceed as before
    let cell_index = CellIndex::try_from(index)
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
pub fn cellToBoundary(h3_index: &str) -> Result<JsValue, JsValue> {
    let index = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(index)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let boundary = cell_index.boundary();

    let boundary_array = js_sys::Array::new();

    for latlng in boundary.iter() {
        let coord_array = js_sys::Array::new();
        coord_array.push(&JsValue::from_f64(latlng.lat()));
        coord_array.push(&JsValue::from_f64(latlng.lng()));
        boundary_array.push(&coord_array.into());
    }

    Ok(boundary_array.into())
}


#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn cellToParent(h3_index: &str, res: u8) -> Result<JsValue, JsValue> {
    // Try converting the input h3Index string to a CellIndex
    let h3_index_u64 = u64::from_str_radix(h3_index, 16)
        .map_err(|e| JsValue::from_str(&format!("Invalid hex string: {}", e)))?;

    let cell_index = CellIndex::try_from(h3_index_u64)
        .map_err(|e| JsValue::from_str(&format!("Invalid H3 index: {}", e)))?;

    // Try converting the resolution
    let resolution = Resolution::try_from(res)
        .map_err(|e| JsValue::from_str(&format!("Invalid resolution: {}", e)))?;

    // Get the parent cell index at the given resolution
    let parent = cell_index
        .parent(resolution)
        .ok_or_else(|| JsValue::from_str("Failed to find parent at given resolution"))?;

    // Return the parent cell index as a string
    Ok(JsValue::from_str(&parent.to_string()))
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
pub fn polygonToCells(polygon_js: &JsValue, resolution_js: u8) -> Result<JsValue, JsValue> {
    // Convert JavaScript resolution input to Rust type
    let resolution = Resolution::try_from(resolution_js)
        .map_err(|_| JsValue::from_str("Invalid resolution"))?;

    // Deserialize the input JS array to a Vec<Vec<f64>> for lat, lng pairs
    let polygon_coords: Vec<Vec<f64>> = from_value(polygon_js.clone())
        .map_err(|_| JsValue::from_str("Failed to parse polygon coordinates"))?;

    // Convert the Vec<Vec<f64>> to geo::Polygon<f64>
    let exterior_coords: Vec<Coord<f64>> = polygon_coords.iter()
        .map(|pair| Coord { x: pair[1], y: pair[0] }) // Note: geo::Coord is in x, y order, which corresponds to lng, lat
        .collect();
    let geo_polygon = GeoPolygon::new(LineString(exterior_coords), vec![]);

    // Convert geo::Polygon<f64> to Polygon
    let polygon = Polygon::from_degrees(geo_polygon)
        .map_err(|_| JsValue::from_str("Failed to create polygon from coordinates"))?;

    let config = PolyfillConfig::new(resolution);

    let cells_iter: Box<dyn Iterator<Item = CellIndex>> = Box::new(polygon.to_cells(config));

    let cells_array = cells_iter.map(|cell_index| JsValue::from_str(&cell_index.to_string()))
        .collect::<Array>();

    Ok(JsValue::from(cells_array))
}

