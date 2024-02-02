<template>
  <nav class="navbar">
    <div class="navbar-content">
      <div class="brand">
        <h1 class="brand-title">divert</h1>
        <span class="brand-subtitle">Spotify migration tool</span>
      </div>
      <button class="help-button">?</button>
    </div>
  </nav>
</template>

<script>
export default {
  name: "Navbar",
};
</script>

<style scoped>
.navbar {
  width: 100%;
  position: fixed;
  top: 0;
  left: 0;
  background-color: #333;
  color: white;
  padding: 0.3rem 0;
  z-index: 1000;
  margin-bottom: 0; /* 确保没有额外的底部边距 */
}

.navbar-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 90%;
  margin: 0 auto;
}

.brand {
  display: flex;
  align-items: center;
  margin-left: 6%;
  position: relative; /* 添加相对定位作为基准 */
}
.brand-title {
  font-size: 1.5rem;
}

.brand-subtitle {
  color: grey;
  margin-left: 0.5rem;
  font-size: 0.8rem;
  position: relative; /* 添加相对定位 */
  top: 2px; /* 向下移动 subtitle */
}

.help-button {
  border: none;
  background-color: transparent;
  color: white;
  font-size: 1.5rem;
  cursor: pointer;
  margin-right: 6%;
}
</style>
