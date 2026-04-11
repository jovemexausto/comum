const path = require('path')
const { getDefaultConfig } = require('expo/metro-config')

const config = getDefaultConfig(__dirname)

config.watchFolders = [path.resolve(__dirname, '../../packages/comum-js')]
config.resolver.nodeModulesPaths = [
  path.resolve(__dirname, 'node_modules'),
  path.resolve(__dirname, '../../node_modules'),
]

// Mute spurious @noble/hashes export warning in terminal
// It successfully falls back to file resolution, so it's safe to ignore.
const originalWarn = console.warn;
console.warn = (...args) => {
  if (
    args[0] &&
    typeof args[0] === "string" &&
    args[0].includes("@noble/hashes") &&
    args[0].includes('not listed in the "exports"')
  ) {
    return;
  }
  originalWarn(...args);
};

module.exports = config
