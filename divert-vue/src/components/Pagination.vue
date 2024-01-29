<template>
  <nav class="pagination" role="navigation" aria-label="pagination">
    <a class="pagination-previous" @click="changePage(currentPage - 1)" :disabled="currentPage <= 1">Previous</a>
    <a class="pagination-next" @click="changePage(currentPage + 1)" :disabled="currentPage >= totalPages">Next</a>
    <ul class="pagination-list">
      <li v-for="page in totalPages" :key="page">
        <a class="pagination-link" :class="{ 'is-current': page === currentPage }" @click="changePage(page)">
          {{ page }}
        </a>
      </li>
    </ul>
  </nav>
</template>

<script>
export default {
  name: 'Pagination',
  props: {
    totalPages: {
      type: Number,
      required: true,
    },
  },
  data() {
    return {
      currentPage: 1,
    };
  },
  methods: {
    changePage(page) {
      if (page >= 1 && page <= this.totalPages) {
        this.currentPage = page;
        this.$emit('page-changed', page);
      }
    },
  },
};
</script>

<style scoped>
/* 这里添加CSS样式 */
</style>
