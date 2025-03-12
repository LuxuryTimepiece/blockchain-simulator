<template>
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-4 bg-gradient-to-r from-blue-500 to-purple-500 text-white p-2 rounded">
      Blockchain Simulator
    </h1>
    <form @submit.prevent="addBlock" class="space-y-4">
      <input
        v-model="blockData"
        type="text"
        placeholder="Enter block data"
        class="border p-2 rounded w-full"
      />
      <button type="submit" class="bg-blue-500 text-white p-2 rounded hover:bg-blue-600">
        Add Block
      </button>
    </form>
    <div class="mt-4 h-164 overflow-auto border rounded shadow-md"> <!-- Scrollable container -->
      <table class="min-w-full bg-white">
        <thead>
          <tr class="bg-gray-200">
            <th class="py-2 px-4">Index</th>
            <th class="py-2 px-4">Data</th>
            <th class="py-2 px-4">Hash</th>
            <th class="py-2 px-4">Mining Time (ms)</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="block in blocks" :key="block.index" class="border-t">
            <td class="py-2 px-4">{{ block.index }}</td>
            <td class="py-2 px-4">{{ block.data }}</td>
            <td class="py-2 px-4">{{ block.hash }}</td>
            <td class="py-2 px-4">{{ block.mining_time_ms }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <div class="mt-4 h-64">
      <BarChart :data="chartData" :options="chartOptions" :key="chartUpdateKey" />
    </div>
    <p :class="isValid ? 'text-green-500' : 'text-red-500'">
      Blockchain is {{ isValid ? 'Valid' : 'Invalid' }}
    </p>
  </div>
</template>

<script>
import axios from 'axios';
import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

export default {
  name: 'App',
  components: {
    BarChart: Bar,
  },
  data() {
    return {
      blockData: '',
      blocks: [],
      isValid: false,
      chartData: {
        labels: [],
        datasets: [
          {
            label: 'Mining Time (ms)',
            backgroundColor: '#36A2EB',
            data: [],
          },
        ],
      },
      chartOptions: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true,
            title: {
              display: true,
              text: 'Mining Time (ms)',
            },
          },
          x: {
            title: {
              display: true,
              text: 'Block Index',
            },
          },
        },
      },
      chartUpdateKey: 0,
    };
  },
  methods: {
    async addBlock() {
      try {
        const response = await axios.post('http://localhost:3000/api/blocks', {
          data: this.blockData,
        }, {
          headers: {
            'Content-Type': 'application/json',
          },
        });
        this.blocks = [...this.blocks, response.data];
        this.blockData = '';
        this.updateChart();
        this.chartUpdateKey += 1;
        await this.fetchValidation();
      } catch (error) {
        console.error('Failed to add block:', error);
      }
    },
    async fetchBlocks() {
      try {
        const response = await axios.get('http://localhost:3000/api/blocks');
        this.blocks = response.data;
        this.updateChart();
      } catch (error) {
        console.error('Failed to fetch blocks:', error);
      }
    },
    async fetchValidation() {
      try {
        const response = await axios.get('http://localhost:3000/api/validate');
        this.isValid = response.data;
      } catch (error) {
        console.error('Failed to fetch validation:', error);
      }
    },
    updateChart() {
      this.chartData.labels = this.blocks.map(block => `Block ${block.index}`);
      this.chartData.datasets[0].data = this.blocks.map(block => block.mining_time_ms || 0);
    },
  },
  mounted() {
    this.fetchBlocks();
    this.fetchValidation();
  },
};
</script>

<style scoped>
/* Ensure table cells don’t overflow horizontally */
table {
  width: 100%;
  table-layout: fixed;
}
th, td {
  word-wrap: break-word;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>