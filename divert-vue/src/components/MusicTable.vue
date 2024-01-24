<template>
    <v-container>
      <v-row>
        <v-col cols="12">
          <v-card>
            <v-card-title>Tracks</v-card-title>
            <v-data-table
              :headers="headers"
              :items="tracks"
              :items-per-page="10"
              class="elevation-1"
            ></v-data-table>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </template>
  
  <script>
  export default {
    data() {
      return {
        headers: [
          { text: '#', value: 'number' },
          { text: 'Title', value: 'title' },
          { text: 'Artist', value: 'artist' },
          { text: 'Album', value: 'album' },
        ],
        tracks: [
          // 此处添加音乐曲目数据
        ],
      };
    },
  };
  </script>
  