<template>
  <div>
    <table class="table is-fullwidth">
      <thead>
        <tr>
          <th><input type="checkbox"></th>
          <th>#</th>
          <th>Name</th>
          <th>Tracks</th>
          <th>Public</th>
          <th>Collaborative</th>
          <th>Owner</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(playlist, index) in playlists" :key="playlist.id">
          <td><input type="checkbox"></td>
          <td>{{ index + 1 }}</td>
          <td>{{ playlist.name }}</td>
          <td>{{ playlist.tracks }}</td>
          <td>{{ playlist.public ? '✓' : '✕' }}</td>
          <td>{{ playlist.collaborative ? '✓' : '✕' }}</td>
          <td>{{ playlist.owner }}</td>
        </tr>
      </tbody>
    </table>
    <Pagination :total-pages="totalPages" @page-changed="fetchPlaylists" />
  </div>
</template>

<script>
import axios from 'axios';
import Pagination from './Pagination.vue';

export default {
  name: 'PlaylistTable',
  components: {
    Pagination,
  },
  data() {
    return {
      playlists: [],
      totalPages: 0,
    };
  },
  created() {
    this.fetchPlaylists(1);
  },
  methods: {
    async fetchPlaylists(page) {
      try {
        // 这里替换成实际的API端点
        const response = await axios.get(`/api/playlists?page=${page}`);
        this.playlists = response.data.items;
        this.totalPages = response.data.totalPages;
      } catch (error) {
        console.error('There was an error fetching the playlists:', error);
      }
    },
  },
};
</script>

<style scoped>
/* 这里添加CSS样式 */
</style>
