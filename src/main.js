import Vue from 'vue'
import App from './App.vue'
import './registerServiceWorker'
import router from './router'
import store from './store'
const WasmPack = import('../pkg/index.js')
Vue.prototype.$wasm = {}
Vue.prototype.$budget = {}

Vue.config.productionTip = false

WasmPack
  .then(m => {
    Vue.prototype.$wasm = m;
    //let t = m.net2("./test-small.csv")
    //  .then( data => {
    //    Vue.prototype.$wasm.budget = data
        console.log("MAIN",m)

        new Vue({
          router,
          store,
          render: h => h(App)
        }).$mount('#app')
      })
    //})
