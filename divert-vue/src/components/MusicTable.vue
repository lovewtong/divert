<template>
  <v-container>
    <!-- Table actions -->
    <v-row>
      <v-col>
        <v-btn @click="transferAll">Transfer all</v-btn>
        <v-btn @click="transferSelected">Transfer selected</v-btn>
        <!-- Add additional actions as needed -->
      </v-col>
    </v-row>

    <!-- Music Table -->
    <v-row>
      <v-col>
        <v-simple-table>
          <template v-slot:default>
            <thead>
              <tr>
                <th class="text-left">
                  <v-checkbox
                    v-model="selectAll"
                    :indeterminate="selected.length > 0 && selected.length < playlists.length"
                    @click.stop="toggleAll"
                  ></v-checkbox>
                </th>
                <th class="text-left">#</th>
                <th class="text-left">Name</th>
                <th class="text-left">Tracks</th>
                <th class="text-left">Public</th>
                <th class="text-left">Collaborative</th>
                <th class="text-left">Owner</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(playlist, index) in playlists" :key="playlist.id">
                <td><v-checkbox v-model="selected" :value="playlist.id"></v-checkbox></td>
                <td>{{ index + 1 }}</td>
                <td>{{ playlist.name }}</td>
                <td>{{ playlist.tracks }}</td>
                <td>{{ playlist.public ? '✔️' : '✖️' }}</td>
                <td>{{ playlist.collaborative ? '✔️' : '✖️' }}</td>
                <td>{{ playlist.owner }}</td>
              </tr>
            </tbody>
          </template>
        </v-simple-table>
      </v-col>
    </v-row>

    <!-- Pagination -->
    <v-row justify="center">
      <v-pagination
        v-model="page"
        :length="pageCount"
      ></v-pagination>
    </v-row>
  </v-container>
</template>

<script>
export default {
  data() {
    return {
      selectAll: false,
      selected: [],
      playlists: [], // You'll need to populate this with your playlist data
      page: 1,
      pageCount: 4 // Example page count
    };
  },
  methods: {
    toggleAll() {
      if (this.selectAll) {
        this.selected = [];
      } else {
        this.selected = this.playlists.map(p => p.id);
      }
      this.selectAll = !this.selectAll;
    },
    transferAll() {
      // Logic for transferring all playlists
    },
    transferSelected() {
      // Logic for transferring selected playlists
    }
  }
};
</script>
