import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Define __dirname for ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Define paths
const pkgDir = path.join(__dirname, 'pkg');
const oldName = path.join(pkgDir, 'h3o_wasm.d.ts');
const newName = path.join(pkgDir, 'h3o_wasm_wrapper.d.ts');

// Rename the output file
fs.renameSync(oldName, newName);

// Update package.json
const packageJsonPath = path.join(pkgDir, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
packageJson.main = './h3o_wasm_wrapper.js';
packageJson.module = './h3o_wasm_wrapper.js';
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

console.log('Build customization completed.');
