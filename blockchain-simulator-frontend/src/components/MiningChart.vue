<template>
  <div class="mt-4 h-64">
    <Chart v-if="blocks.length > 0" :type="chartType" :data="chartData" :options="chartOptions" />
    <p v-else>Loading chart data...</p>
  </div>
</template>

<script>
import { Chart } from 'vue-chartjs';
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  BarController, // Add this
  BarElement,
  CategoryScale,
  LinearScale,
} from 'chart.js';

// Register all necessary components, including BarController
ChartJS.register(Title, Tooltip, Legend, BarController, BarElement, CategoryScale, LinearScale);

export default {
  name: 'MiningChart',
  components: { Chart },
  props: {
    blocks: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      chartType: 'bar',
    };
  },
  computed: {
    chartData() {
      const data = {
        labels: this.blocks.map(block => `Block ${block.index}`),
        datasets: [
          {
            label: 'Mining Time (ms)',
            backgroundColor: '#42A5F5',
            data: this.blocks.map(block => block.mining_time_ms || 0),
          },
        ],
      };
      console.log('Chart Data:', data);
      return data;
    },
    chartOptions() {
      return {
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
      };
    },
  },
};
</script>

<style scoped>
canvas {
  border: 1px solid #ccc; /* Optional: for visibility */
}
</style>