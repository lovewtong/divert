<template>
  <div id="app">
    <Navbar />
    <LoginBox />
    <!-- 其他组件将会在这里添加 -->
  </div>
</template>

<script>
// 导入组件
import Navbar from './components/Navbar.vue'
import LoginBox from './components/LoginBox.vue'

export default {
  name: 'App',
  components: {
    // 注册组件
    Navbar,
    LoginBox
  }
}
</script>

<style>
.content {
  display: flex;
  justify-content: center; /* 水平居中 */
  padding: 2rem; /* 或者你需要的任何值 */
}
</style>
