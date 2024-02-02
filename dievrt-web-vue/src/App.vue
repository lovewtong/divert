<template>
  <div id="app">
    <Navbar />
    <LoginBox />
    <tabs />
    <!-- 其他组件 -->
  </div>
</template>

<script>
// 导入组件
import Navbar from './components/Navbar.vue'
import LoginBox from './components/LoginBox.vue'
import tabs from './components/list.vue'

export default {
  name: 'App',
  components: {
    // 注册组件
    Navbar,
    LoginBox,
    tabs
  }
}
</script>

<style>
.content {
  display: flex;
  justify-content: center; /* 水平居中 */
  padding: 2rem;
}
</style>
