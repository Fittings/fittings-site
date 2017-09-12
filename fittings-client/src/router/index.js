import Vue from 'vue'
import Router from 'vue-router'
import Galleries from '@/components/Galleries'
import VueResource from 'vue-resource'

Vue.use(VueResource)
Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      redirect: '/galleries'
    },
    {
      path: '/galleries',
      name: 'Galleries',
      component: Galleries
    }
  ]
})
