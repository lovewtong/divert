<template>
  <div>
    <section class="section">
      <ArtistTable />
    </section>
  </div>
</template>
  
  <script>
import ArtistTable from '@/components/ArtistTable.vue'

export default {
  name: 'ArtistsView',
  components: {
    ArtistTable
  }
}
</script>
  