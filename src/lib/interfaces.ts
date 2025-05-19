export interface DataPoint {
  timestamp: number
  cpu_usage: number
  memory: number
  processes: Process[]
}

export interface DataRecord {
  time: number
  value: number
}

export interface Process {
  pid: number
  name: string
  cpu: number
  memory: number
}
