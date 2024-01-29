<template>
    <div>
      <table class="table is-fullwidth">
        <thead>
          <tr>
            <th>#</th>
            <th>Artist</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(artist, index) in artists" :key="artist.id">
            <td>{{ index + 1 }}</td>
            <td>{{ artist.name }}</td>
          </tr>
        </tbody>
      </table>
      <Pagination :total-pages="totalPages" @page-changed="fetchArtists" />
    </div>
  </template>
  
  <script>
  import axios from 'axios';
  import Pagination from './Pagination.vue';
  
  export default {
    name: 'ArtistTable',
    components: {
      Pagination,
    },
    data() {
      return {
        artists: [],
        totalPages: 0,
      };
    },
    created() {
      this.fetchArtists(1);
    },
    methods: {
      async fetchArtists(page) {
        try {
          // 这里替换成实际的API端点
          const response = await axios.get(`/api/artists?page=${page}`);
          this.artists = response.data.items;
          this.totalPages = response.data.totalPages;
        } catch (error) {
          console.error('There was an error fetching the artists:', error);
        }
      },
    },
  };
  </script>
  
  <style scoped>
  /* 这里添加CSS样式 */
  </style>
  