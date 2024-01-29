<template>
  <div>
    <table class="table is-fullwidth">
      <thead>
        <tr>
          <th>#</th>
          <th>Title</th>
          <th>Artist</th>
          <th>Album</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(track, index) in tracks" :key="track.id">
          <td>{{ index + 1 }}</td>
          <td>{{ track.title }}</td>
          <td>{{ track.artist }}</td>
          <td>{{ track.album }}</td>
        </tr>
      </tbody>
    </table>
    <Pagination :total-pages="totalPages" @page-changed="fetchTracks" />
  </div>
</template>

<script>
import axios from 'axios';
import Pagination from './Pagination.vue';

export default {
  name: 'TrackTable',
  components: {
    Pagination,
  },
  data() {
    return {
      tracks: [],
      totalPages: 0,
    };
  },
  created() {
    this.fetchTracks(1);
  },
  methods: {
    async fetchTracks(page) {
      try {
        // 这里替换成实际的API端点
        const response = await axios.get(`/api/tracks?page=${page}`);
        this.tracks = response.data.items;
        this.totalPages = response.data.totalPages;
      } catch (error) {
        console.error('There was an error fetching the tracks:', error);
      }
    },
  },
};
</script>

<style scoped>
/* 这里添加CSS样式 */
</style>
