import Vue from 'vue'
import Router from 'vue-router'
import HomeView from '@/components/HomeView'
import LimbView from '@/components/LimbView'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Home',
      component: HomeView
    },
    {
      path: '/Limb/:limb',
      name: 'Limb',
      component: LimbView
    }
  ]
})
