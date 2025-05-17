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
import { ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from '@/components/ui/context-menu'
import { X } from 'lucide-vue-next'

interface Process {
  pid: number
  name: string
  cpu: number
  memory: number
}

const processes: Ref<Process[]> = ref([
  {
    pid: 1000,
    name: 'chromium',
    cpu: 50.0,
    memory: 256,
  }
])
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
                <ContextMenuItem class="hover:cursor-pointer">
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
