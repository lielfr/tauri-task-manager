<script setup lang="ts">
import TheProcessList from '@/components/the-process-list.vue'
import UsageChart from '@/components/usage-chart.vue'

const dummyData = [
  {
    time: 1,
    value: 0.5,
  },
  {
    time: 2,
    value: 0.7,
  },
  {
    time: 3,
    value: 1.0,
  },
]
</script>

<template>
  <main class="grid md:grid-cols-2 gap-4">
    <usage-chart title="CPU" :data="dummyData" color="var(--chart-1)"></usage-chart>
    <usage-chart title="Memory" :data="dummyData" color="var(--chart-2)"></usage-chart>
    <the-process-list class="md:col-span-2"></the-process-list>
  </main>
</template>
