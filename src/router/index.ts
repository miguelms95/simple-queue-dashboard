import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Queues from '../views/Queues.vue';
import Messages from '../views/Messages.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/queues',
      name: 'queues',
      component: Queues
    },
    {
      path: '/messages/:queueUrl',
      name: 'messages',
      component: Messages
    }
  ]
});

export default router;
