

<style scoped>
.login-box {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  margin-top: 1rem; /* 移除上边距 */
  background-color: #eeeeec;
  border-radius: 5px;
  min-width: 300px;
  max-width: 300px;
  width: 80%;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  /* position: relative; /* 可以改为 absolute 如果 Navbar 是 fixed */
  /* z-index: 10; */
  left: 10%; /* 靠左对齐 */
  right: 0; /* 与左边界同样的做法，保证宽度一致性 */
  position: absolute; /* 如果 Navbar 是 fixed 的话 */
  top: 50px; /* Navbar 高度加上想要的空间 */
  z-index: 10;
}
.user-info,
.login-button {
  cursor: pointer;
}

.old-user {
  font-size: 0.9rem;
  color: grey;
}
</style>


<template>
  <div class="login-box">
    <div v-if="isLoggedIn" class="user-info">
      {{ username }}
      <button @click="signOut">sign out</button>
    </div>
    <div v-else class="login-button" @click="login">sign in</div>
    <div class="old-user">Old user</div>
  </div>
</template>

<script>
export default {
  name: "LoginBox",
  data() {
    return {
      isLoggedIn: false,
      username: "",
      loginWindow: null,
    };
  },
  methods: {
    login() {
      const loginUrl = "http://localhost:8080/login/source"; // 后端登录URL
      this.loginWindow = window.open(
        loginUrl,
        "_blank",
        "width=800,height=600"
      );

      // 监听从打开的登录窗口传来的消息
      window.addEventListener(
        "message",
        (event) => {
          if (event.origin === "") {
            // 替换为实际的前端URL
            if (event.data.display_name) {
              this.isLoggedIn = true;
              this.username = event.data.display_name;
            }
          }
        },
        false
      );
    },
    signOut() {
      this.isLoggedIn = false;
      this.username = "";
      // 在这里添加向后端发送登出请求的代码
    },
  },
};
</script>
