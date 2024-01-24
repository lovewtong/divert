import { createApp } from 'vue';
import App from './App.vue';
import router from './router'; // Ensure you have router/index.js configured
import store from './stores'; // Ensure you have store/index.js configured with Vuex 4

// If you're using Vuetify or another UI framework, import and set it up here
// import vuetify from './plugins/vuetify'; // This is an example for Vuetify

const app = createApp(App);

// Use router and store
app.use(router);
app.use(store);

// Use Vuetify, if applicable
app.use(vuetify);

// Mount the application
app.mount('#app');
