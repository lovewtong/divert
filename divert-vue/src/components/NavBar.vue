<template>
  <nav class="navbar">
    <div class="navbar-brand">
      <a class="navbar-item" href="/">Music Transfer</a>
    </div>
    <div class="navbar-menu">
      <div class="navbar-start">
        <a class="navbar-item" href="/playlists">Playlists</a>
        <a class="navbar-item" href="/tracks">Tracks</a>
        <a class="navbar-item" href="/albums">Albums</a>
        <a class="navbar-item" href="/artists">Artists</a>
      </div>
      <div class="navbar-end">
        <div class="navbar-item">
          <div class="buttons">
            <a class="button is-light" @click="login">Sign in</a>
            <a class="button is-primary" @click="logout">Sign out</a>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'NavBar',
  methods: {
    login() {
      // 这里将触发登录流程
    },
    logout() {
      // 这里将处理注销流程
    },
  },
};
</script>

<style scoped>
/* 这里添加CSS样式 */
</style>
