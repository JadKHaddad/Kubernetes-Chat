import { createRouter, createWebHistory } from 'vue-router'
import Signin from '../views/Signin.vue'
import Home from '../views/Home.vue'
import Chat from '../views/Chat.vue'

const routes = [
  {
    path: '/',
    name: 'Signin',
    component: Signin,
    //props: true
  },
  {
    path: '/home',
    name: 'Home',
    component: Home,
    //props: true
  },
  {
    path: '/chat/:username',
    name: 'Chat',
    component: Chat,
    props: true
  },

]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
