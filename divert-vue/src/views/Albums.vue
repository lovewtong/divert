<template>
  <div>
    <section class="section">
      <AlbumTable />
    </section>
  </div>
</template>
  
  <script>
import AlbumTable from '@/components/AlbumTable.vue'

export default {
  name: 'AlbumsView',
  components: {
    AlbumTable
  }
}
</script>
  