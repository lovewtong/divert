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
    async login() {
      const loginUrl = "http://localhost:8080/login/source"; // 后端登录URL
      // 打开新窗口进行登录
      this.loginWindow = window.open(
        loginUrl,
        "_blank",
        "width=800,height=600"
      );

      // 检查登录窗口是否关闭，如果关闭则调用getUser
      const checkLoginWindowInterval = setInterval(() => {
        if (this.loginWindow.closed) {
          clearInterval(checkLoginWindowInterval);
          this.getUser(); // 调用getUser获取用户信息
        }
      }, 1000);
    },
    async getUser() {
      try {
        // 首先，从服务器获取访问令牌
        const tokenResponse = await fetch(
          "http://localhost:8080/get_access_token",
          {
            credentials: "include",
          }
        );
        const tokenData = await tokenResponse.json();
        const accessToken = tokenData.access_token_source;

        // 然后使用访问令牌发起getUser请求
        const response = await fetch("http://localhost:8080/getUser", {
          method: "GET",
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
          credentials: "include", // 依赖于cookies
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        } else {
          const data = await response.json();
          this.username = data.display_name;
          this.isLoggedIn = true;
        }
      } catch (error) {
        console.error("Fetch操作出现问题：", error);
      }
    },

    signOut() {
      // 登出逻辑
      this.isLoggedIn = false;
      this.username = "";
    },
  },
};
</script>