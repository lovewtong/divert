<template>
  <div>
    <!-- Tab headers -->
    <div class="tabs">
      <div @click="selectTab('playlists')">Playlists</div>
      <div @click="selectTab('tracks')">Tracks</div>
      <div @click="selectTab('albums')">Albums</div>
      <div @click="selectTab('artists')">Artists</div>
    </div>

    <!-- Tab content -->
    <div v-if="selectedTab === 'playlists'">
      <!-- Playlists content -->
    </div>
    <div v-if="selectedTab === 'tracks'">
      <!-- Tracks content -->
    </div>
    <div v-if="selectedTab === 'albums'">
      <!-- Albums content -->
    </div>
    <div v-if="selectedTab === 'artists'">
      <!-- Artists content -->
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      selectedTab: 'playlists', // 默认选中的选项卡
      // 其他数据...
    };
  },
  methods: {
    selectTab(tab) {
      this.selectedTab = tab;
    },
    // 从后端获取数据的方法...
  },
  mounted() {
    // 当组件加载时获取所有数据
    this.fetchPlaylists();
    this.fetchTracks();
    this.fetchAlbums();
    this.fetchArtists();
  }
};
</script>

<style scoped>
.tabs div {
  cursor: pointer;
  display: inline-block;
  padding: 10px;
  /* 其他样式... */
}
</style>
