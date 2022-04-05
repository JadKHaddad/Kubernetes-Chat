module.exports = {
    //publicPath: process.env.NODE_ENV === 'production' ? 'static/' : '/',
    devServer: {
      clientLogLevel: 'info',
      proxy: {
        '/api': {
          target: 'http://localhost:5000/',
          changeOrigin: true,
          ws: true,
          pathRewrite: {
            '^/api': ''
          }
        }
      }
    }
  };