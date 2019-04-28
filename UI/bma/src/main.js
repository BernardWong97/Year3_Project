import Vue from 'vue'
import App from './App.vue'
import router from './router'

import api from '@/utils/api'

Vue.$api = api
Vue.config.productionTip = false

new Vue({
  router,
  render: h => h(App)
}).$mount('#app')

Object.defineProperty(Vue.prototype, '$api', {
  get () {
    return api
  }
})