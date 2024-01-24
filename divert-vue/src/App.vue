<template>
  <v-app>
    <!-- Application Bar -->
    <v-app-bar app>
      <v-toolbar-title>My Music App</v-toolbar-title>
      <v-spacer></v-spacer>
      <!-- Sign In/Out Buttons -->
      <v-btn text v-if="!user" @click="signIn">Sign in</v-btn>
      <v-btn text v-else @click="signOut">Sign out ({{ user }})</v-btn>
    </v-app-bar>

    <!-- Navigation Drawer -->
    <v-navigation-drawer app>
      <v-list dense>
        <v-list-item link v-for="item in menuItems" :key="item.title" @click="navigateTo(item.link)">
          <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <!-- Main content -->
    <v-main>
      <v-container fluid>
        <router-view></router-view>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
export default {
  data: () => ({
    user: null, // This should be replaced with actual user authentication logic
    menuItems: [
      { title: 'Playlists', icon: 'mdi-playlist-music', link: '/playlists' },
      { title: 'Tracks', icon: 'mdi-music-note', link: '/tracks' },
      { title: 'Albums', icon: 'mdi-album', link: '/albums' },
      { title: 'Artists', icon: 'mdi-account-music', link: '/artists' },
      // Add more menu items as needed
    ],
  }),
  methods: {
    signIn() {
      // Placeholder for sign-in logic
      this.user = 'User Name'; // Replace with actual sign-in logic
    },
    signOut() {
      // Placeholder for sign-out logic
      this.user = null; // Replace with actual sign-out logic
    },
    navigateTo(link) {
      this.$router.push(link);
    },
  },
};
</script>

<style>
/* Add your CSS styles here */
</style>
