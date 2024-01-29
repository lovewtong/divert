<template>
  <div>
    <section class="section">
      <TrackTable />
    </section>
  </div>
</template>
  
  <script>
import TrackTable from '@/components/TrackTable.vue'

export default {
  name: 'TracksView',
  components: {
    TrackTable
  }
}
</script>
  