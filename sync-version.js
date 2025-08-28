#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// 读取 package.json
const packageJsonPath = path.join(__dirname, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const version = packageJson.version;

// 获取当前日期 (YYYY-MM-DD 格式)
const currentDate = new Date().toISOString().split('T')[0];

console.log(`📦 同步版本号到 ${version}`);
console.log(`📅 更新日期: ${currentDate}`);

// 创建版本信息对象
const versionInfo = {
    version: version,
    updateDate: currentDate,
    lastSync: new Date().toISOString()
};

// 更新 src-tauri/Cargo.toml
const cargoTomlPath = path.join(__dirname, 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(/version = "[^"]*"/, `version = "${version}"`);
fs.writeFileSync(cargoTomlPath, cargoToml);
console.log('✅ 已更新 src-tauri/Cargo.toml');

// 更新 src-tauri/tauri.conf.json
const tauriConfPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');
let tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
console.log('✅ 已更新 src-tauri/tauri.conf.json');

// 创建版本信息文件
const versionInfoPath = path.join(__dirname, 'src-tauri', 'version-info.json');
fs.writeFileSync(versionInfoPath, JSON.stringify(versionInfo, null, 2));
console.log('✅ 已创建 src-tauri/version-info.json');

console.log('🎉 版本号同步完成！');
