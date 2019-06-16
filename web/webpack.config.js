const CopyPlugin = require('copy-webpack-plugin')
const path = require('path')

module.exports = {
  mode: 'development',
  entry: './src/bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new CopyPlugin(['index.html'])
  ],
}
