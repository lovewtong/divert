import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

// 全局样式文件
// import './assets/styles/main.css';

const app = createApp(App);

app.use(router);

app.mount('#app');
