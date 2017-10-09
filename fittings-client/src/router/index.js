import Vue from 'vue'
import Router from 'vue-router'
import Galleries from '@/components/Galleries'
import Gallery from '@/components/Gallery'
import VueResource from 'vue-resource'

Vue.use(VueResource)
Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/galleries',
      name: 'Galleries',
      component: Galleries
    },
    {
      path: '/gallery/:id',
      name: 'Gallery',
      component: Gallery
    },
    {
      path: '/',
      redirect: '/galleries'
    }
  ]
})
