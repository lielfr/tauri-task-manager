<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { type Ref, ref } from 'vue'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { X } from 'lucide-vue-next'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { DataPoint, Process } from '@/lib/interfaces.ts'

const processes: Ref<Process[]> = ref([])

listen<DataPoint>('data_point', (data) => {
  processes.value = data.payload.processes
})

const killProcess = (pid: number) => {
  invoke('kill_process', { pid })
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Running Processes</CardTitle>
    </CardHeader>
    <CardContent>
      <Table container-class="max-h-[33svh]">
        <TableHeader class="sticky top-0 bg-background">
          <TableRow>
            <TableHead>Name</TableHead>
            <TableHead>PID</TableHead>
            <TableHead>% CPU</TableHead>
            <TableHead>Memory [MB]</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <ContextMenu v-for="process of processes" :key="process.pid">
            <ContextMenuContent class="bg-background text-foreground">
              <ContextMenuItem class="hover:cursor-pointer" @select="killProcess(process.pid)">
                <X />
                Kill Process
              </ContextMenuItem>
            </ContextMenuContent>
            <ContextMenuTrigger as-child>
              <TableRow>
                <TableCell>{{ process.name }}</TableCell>
                <TableCell>{{ process.pid }}</TableCell>
                <TableCell>{{ process.cpu }}</TableCell>
                <TableCell>{{ process.memory }}</TableCell>
              </TableRow>
            </ContextMenuTrigger>
          </ContextMenu>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>

<style scoped></style>
