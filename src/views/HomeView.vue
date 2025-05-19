<script setup lang="ts">
import TheProcessList from '@/components/the-process-list.vue'
import UsageChart from '@/components/usage-chart.vue'
import { listen } from '@tauri-apps/api/event'
import { type Ref, ref } from 'vue'

const cpuData: Ref<DataRecord[]> = ref([])
const memoryData: Ref<DataRecord[]> = ref([])

interface DataPoint {
  timestamp: number,
  cpu_usage: number,
  memory: number,
}

interface DataRecord {
  time: number
  value: number
}

listen<DataPoint>('data_point', (data) => {
  cpuData.value.push({
    time: data.payload.timestamp,
    value: data.payload.cpu_usage,
  })

  memoryData.value.push({
    time: data.payload.timestamp,
    value: data.payload.memory,
  })
})
</script>

<template>
  <main class="grid md:grid-cols-2 gap-4">
    <usage-chart title="CPU" :data="cpuData" color="var(--chart-1)"></usage-chart>
    <usage-chart title="Memory" :data="memoryData" color="var(--chart-2)"></usage-chart>
    <the-process-list class="md:col-span-2"></the-process-list>
  </main>
</template>
