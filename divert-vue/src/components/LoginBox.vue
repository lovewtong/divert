<template>
  <div class="login-box">
    <div v-if="isLoggedIn" class="user-info">
      {{ username }} sign out
    </div>
    <div v-else class="login-button" @click="login">
      sign in
    </div>
    <div class="old-user">
      Old user
    </div>
  </div>
</template>

<script>
export default {
  name: 'LoginBox',
  data() {
    return {
      isLoggedIn: false, // 假设用户初始时未登录
      username: 'user', // 这里应该是从会话中获取的用户名
    };
  },
  methods: {
    login() {
      // 这里应该是触发登录流程的方法
      console.log('Login process start');
      // 你可以在这里调用 login_source 函数或是跳转到登录页面
    },
    signOut() {
      // 这里应该是触发登出流程的方法
      console.log('Sign out process start');
      // 实现用户登出逻辑
    }
  },
  mounted() {
    console.log(this.$el); // 这将打印出根元素，你可以在开发者工具的控制台中检查它
  }
};

  

</script>

<style scoped>
.login-box {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  padding: 1rem;
  background-color: #161603;
  border-radius: 20px;
}

.user-info, .login-button {
  cursor: pointer;
}

.old-user {
  font-size: 0.9rem;
  color: grey;
}
</style>
