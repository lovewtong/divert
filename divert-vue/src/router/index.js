import { createRouter, createWebHistory } from 'vue-router';

// Import your components
import Playlists from '@/components/Playlists.vue';
import Tracks from '@/components/Tracks.vue';
import Albums from '@/components/Albums.vue';
import Artists from '@/components/Artists.vue';
import PlaylistTable from '@/components/MusicTable.vue';

const routes = [
  {
    path: '/',
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
  {
    path: '/playlist-table',
    name: 'PlaylistTable',
    component: PlaylistTable
  },
  // Add other routes as needed
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
