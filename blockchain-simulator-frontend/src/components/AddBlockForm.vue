<template>
  <form @submit.prevent="addBlock" class="space-y-4">
    <input v-model="data" type="text" placeholder="Enter block data" class="border p-2 rounded w-full" />
    <button type="submit" class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600">Add Block</button>
  </form>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return { data: '' };
  },
  methods: {
    async addBlock() {
      try {
        await axios.post('/api/add', { data: this.data });
        this.data = '';
        this.$emit('block-added');
      } catch (e) {
        console.error('Failed to add block:', e);
      }
    },
  },
};
</script>