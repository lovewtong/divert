<template>
    <div>
      <table class="table is-fullwidth">
        <thead>
          <tr>
            <th>#</th>
            <th>Name</th>
            <th>Artist</th>
            <th>Total Tracks</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(album, index) in albums" :key="album.id">
            <td>{{ index + 1 }}</td>
            <td>{{ album.name }}</td>
            <td>{{ album.artist }}</td>
            <td>{{ album.total_tracks }}</td>
          </tr>
        </tbody>
      </table>
      <Pagination :total-pages="totalPages" @page-changed="fetchAlbums" />
    </div>
  </template>
  
  <script>
  import axios from 'axios';
  import Pagination from './Pagination.vue';
  
  export default {
    name: 'AlbumTable',
    components: {
      Pagination,
    },
    data() {
      return {
        albums: [],
        totalPages: 0,
      };
    },
    created() {
      this.fetchAlbums(1);
    },
    methods: {
      async fetchAlbums(page) {
        try {
          // 这里替换成实际的API端点
          const response = await axios.get(`/api/albums?page=${page}`);
          this.albums = response.data.items;
          this.totalPages = response.data.totalPages;
        } catch (error) {
          console.error('There was an error fetching the albums:', error);
        }
      },
    },
  };
  </script>
  
  <style scoped>
  /* 这里添加CSS样式 */
  </style>
  