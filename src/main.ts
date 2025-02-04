import * as h3o from '../pkg/h3o_wasm_wrapper.js'
import * as h3 from 'h3-js'


function run() {
  console.log(h3o?.UNITS);
  const h3Index = h3.latLngToCell(37.3615593, -122.0553238, 7);
  // get the center of the grandparent cell
  const h3OriginIndex = h3.cellToCenterChild(h3.cellToParent(h3Index, 5), 7);
  const edge = "115283473fffffff";
  const lat = 40.689247;
  const lng = -74.044502;
  const resolution = 9;
  const polygon = [
    [37.813318999983238, -122.4089866999972145],
    [37.7198061999978478, -122.3544736999993603],
    [37.8151571999998453, -122.4798767000009008],
  ];

  const updateTable = (id: string, value: string) => {
    const div =document.createElement("div");
    div.className = "result";
    div.textContent = value;
    document.getElementById(id).innerHTML = "";
    document.getElementById(id).appendChild(div);
  };

  const measureTime = (fn: Function) => {
    const start = performance.now();
    for (let i = 0; i < 1000; i++) {
      fn();
    }
    const end = performance.now();
    const result = fn();
    return { time: end - start, result };
  };

  // Operations
  const operations = [
    {
      name: "isValidCell",
      wasmId: "h3o-is-valid-cell",
      jsId: "h3-is-valid-cell",
      wasmFn: () => h3o.isValidCell(h3Index),
      jsFn: () => h3.isValidCell(h3Index),
    },
    {
      name: "cellArea",
      wasmId: "h3o-cell-area",
      jsId: "h3-cell-area",
      wasmFn: () => h3o.cellArea(h3Index, "km2"),
      jsFn: () => h3.cellArea(h3Index, h3.UNITS.km2),
    },
    {
      name: "cellToLatLng",
      wasmId: "h3o-cell-to-latlng",
      jsId: "h3-cell-to-latlng",
      wasmFn: () => h3o.cellToLatLng(h3Index),
      jsFn: () => h3.cellToLatLng(h3Index),
    },
    {
      name: "latLngToCell",
      wasmId: "h3o-latlng-to-cell",
      jsId: "h3-latlng-to-cell",
      wasmFn: () => h3o.latLngToCell(lat, lng, resolution),
      jsFn: () => h3.latLngToCell(lat, lng, resolution),
    },
    {
      name: "getResolution",
      wasmId: "h3o-get-resolution",
      jsId: "h3-get-resolution",
      wasmFn: () => h3o.getResolution(h3Index),
      jsFn: () => h3.getResolution(h3Index),
    },
    {
      name: "isPentagon",
      wasmId: "h3o-is-pentagon",
      jsId: "h3-is-pentagon",
      wasmFn: () => h3o.isPentagon(h3Index),
      jsFn: () => h3.isPentagon(h3Index),
    },
    {
      name: "edgeLength",
      wasmId: "h3o-edge-length",
      jsId: "h3-edge-length",
      wasmFn: () => h3o.edgeLength(edge, "km"),
      jsFn: () => h3.edgeLength(edge, "km"),
    },
    {
      name: "polygonToCells",
      wasmId: "h3o-polygon-to-cells",
      jsId: "h3-polygon-to-cells",
      wasmFn: () => h3o.polygonToCells(polygon, resolution),
      jsFn: () => h3.polygonToCells(polygon, resolution),
    },
    {
      name: "cellToParent",
      wasmId: "h3o-cell-to-parent",
      jsId: "h3-cell-to-parent",
      wasmFn: () => h3o.cellToParent(h3Index, resolution - 2),
      jsFn: () => h3.cellToParent(h3Index, resolution - 2),

    },
    {
      name: "cellToBoundary",
      wasmId: "h3o-cell-to-boundary",
      jsId: "h3-cell-to-boundary",
      wasmFn: () => h3o.cellToBoundary(h3Index),
      jsFn: () => h3.cellToBoundary(h3Index),
    },
    {
      name: "cellToChildren",
      wasmId: "h3o-cell-to-children",
      jsId: "h3-cell-to-children",
      wasmFn: () => h3o.cellToChildren(h3Index, resolution + 1),
      jsFn: () => h3.cellToChildren(h3Index, resolution + 1),
    },
    {
      name: "cellToCenterChild",
      wasmId: "h3o-cell-to-center-child",
      jsId: "h3-cell-to-center-child",
      wasmFn: () => h3o.cellToCenterChild(h3Index, resolution + 1),
      jsFn: () => h3.cellToCenterChild(h3Index, resolution + 1),
    },
    {
      name: "cellToLocalIj",
      wasmId: "h3o-cell-to-local-ij",
      jsId: "h3-cell-to-local-ij",
      wasmFn: () => h3o.cellToLocalIj(h3OriginIndex, h3Index),
      jsFn: () => h3.cellToLocalIj(h3OriginIndex, h3Index),
    },
    {
      name: "isResClassIII",
      wasmId: "h3o-is-res-class-iii",
      jsId: "h3-is-res-class-iii",
      wasmFn: () => h3o.isResClassIII(h3Index),
      jsFn: () => h3.isResClassIII(h3Index),
    }
  ];

  // Execute operations and update table
  for (const op of operations) {
    const wasm = measureTime(op.wasmFn);
    const js = measureTime(op.jsFn);

    updateTable(`${op.wasmId}-time`, wasm.time.toFixed(2));
    updateTable(`${op.jsId}-time`, js.time.toFixed(2));
    updateTable(`${op.wasmId}-result`, JSON.stringify(wasm.result));
    updateTable(`${op.jsId}-result`, JSON.stringify(js.result));
  }
};

run();