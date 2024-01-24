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
      <v-list>
        <v-list-item-group v-model="drawerItem">
          <v-list-item v-for="(item, i) in menuItems" :key="i" @click="goTo(item.link)">
            <v-list-item-icon>
              <v-icon>{{ item.icon }}</v-icon>
            </v-list-item-icon>
            <v-list-item-content>
              <v-list-item-title>{{ item.text }}</v-list-item-title>
            </v-list-item-content>
          </v-list-item>
        </v-list-item-group>
      </v-list>
    </v-navigation-drawer>

    <!-- Main Content -->
    <v-main>
      <v-container>
        <router-view></router-view>
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
export default {
  data() {
    return {
      user: null, // Replace with your authentication logic
      drawerItem: null,
      menuItems: [
        { text: 'Playlists', icon: 'mdi-playlist-music', link: '/playlists' },
        { text: 'Tracks', icon: 'mdi-music-note', link: '/tracks' },
        { text: 'Albums', icon: 'mdi-album', link: '/albums' },
        { text: 'Artists', icon: 'mdi-account-music', link: '/artists' },
      ],
    };
  },
  methods: {
    signIn() {
      // Replace with your sign in logic
      this.user = 'User';
    },
    signOut() {
      // Replace with your sign out logic
      this.user = null;
    },
    goTo(link) {
      this.$router.push(link);
    },
  },
};
</script>

<style>
/* Add your styles here */
</style>
