import { createRouter, createWebHistory } from 'vue-router'
import Playlists from '../components/Playlists.vue'
import Tracks from '../components/Tracks.vue'
import Albums from '../components/Albums.vue'
import Artists from '../components/Artists.vue'

const routes = [
  { path: '/playlists', component: Playlists },
  { path: '/tracks', component: Tracks },
  { path: '/albums', component: Albums },
  { path: '/artists', component: Artists },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
