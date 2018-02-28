import Vue from 'vue'
import Router from 'vue-router'
import ControlView from '@/components/ControlView'
import LimbView from '@/components/LimbView'
import RiddleView from '@/components/RiddleView'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Control',
      component: ControlView
    },
    {
      path: '/Limb/:limb',
      name: 'Limb',
      component: LimbView
    },
    {
      path: '/Riddle',
      name: 'Riddle',
      component: RiddleView
    }
  ]
})
