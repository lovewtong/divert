import { createRouter, createWebHistory } from 'vue-router';
import Home from '../views/Home.vue';
import Playlists from '../views/Playlists.vue';
import Tracks from '../views/Tracks.vue';
import Albums from '../views/Albums.vue';
import Artists from '../views/Artists.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/playlists',
    name: 'Playlists',
    component: Playlists
  },
  {
    path: '/tracks',
    name: 'Tracks',
    component: Tracks
  },
  {
    path: '/albums',
    name: 'Albums',
    component: Albums
  },
  {
    path: '/artists',
    name: 'Artists',
    component: Artists
  },
  // 其他路由...
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
});

export default router;
