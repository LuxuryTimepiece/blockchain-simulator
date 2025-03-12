<template>
  <table class="min-w-full bg-white shadow-md rounded">
    <thead>
      <tr class="bg-gray-200">
        <th class="py-2 px-4">Index</th>
        <th class="py-2 px-4">Data</th>
        <th class="py-2 px-4">Hash</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="block in blocks" :key="block.index" class="border-t">
        <td class="py-2 px-4">{{ block.index }}</td>
        <td class="py-2 px-4">{{ block.data }}</td>
        <td class="py-2 px-4">{{ block.hash.substring(0, 10) }}...</td>
      </tr>
    </tbody>
  </table>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return { blocks: [] };
  },
  mounted() {
    this.fetchBlocks();
    setInterval(this.fetchBlocks, 5000); // Poll every 5 seconds
  },
  methods: {
    async fetchBlocks() {
      try {
        const res = await axios.get('/api/blocks');
        this.blocks = res.data;
      } catch (e) {
        console.error('Failed to fetch blocks:', e);
      }
    },
  },
};
</script>