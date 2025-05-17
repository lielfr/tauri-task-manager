<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { VisXYContainer, VisAxis, VisArea, VisTooltip, VisCrosshair } from '@unovis/vue'

interface DataRecord {
  time: number
  value: number
}

const { title } = defineProps({
  title: String,
  data: Array<DataRecord>,
  color: {
    type: String,
    default: 'var(--chart-1)',
  },
})

const x = (d: DataRecord) => d.time
const y = (d: DataRecord) => d.value
const template = (d: DataRecord) => `${d.value}`
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>{{ title }}</CardTitle>
    </CardHeader>
    <CardContent>
      <VisXYContainer :data="data">
        <VisArea :x="x" :y="y" :color="color" />
        <VisCrosshair :template="template" />
        <VisAxis type="x" />
        <VisAxis type="y" />
        <VisTooltip />
      </VisXYContainer>
    </CardContent>
  </Card>
</template>

<style scoped></style>
