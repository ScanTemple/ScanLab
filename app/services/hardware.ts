import { invoke } from '@tauri-apps/api/core'

export const useHardwareService = () => {
  return {
    getCpuCount: () => useAsyncData<number>(async () => invoke('get_cpus'), { default: () => 1 }),
  }
}
