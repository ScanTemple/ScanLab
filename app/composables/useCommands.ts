import { invoke } from '@tauri-apps/api/core'
import type { ProcessingStage } from '~~/src-tauri/bindings/ProcessingStage'

export const useCommands = () => {
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Helper function to handle command execution
  const executeCommand = async <T>(
    commandName: string,
    args?: Record<string, unknown>,
  ): Promise<T | null> => {
    try {
      isLoading.value = true
      error.value = null

      const result = await invoke<T>(commandName, args)
      return result
    }
    catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error(`Command ${commandName} failed:`, err)
      return null
    }
    finally {
      isLoading.value = false
    }
  }

  // Project management commands

  // Create a new project with the given name and directory
  const createProject = async (name: string, dir: string): Promise<boolean> => {
    const result = await executeCommand<undefined>('create_project', { name, dir })
    return result !== null
  }

  // Create project in temp directory with default name
  const createTempProject = async (): Promise<boolean> => {
    const result = await executeCommand<undefined>('create_temp_project')
    return result !== null
  }

  // Opens dialog tooad an existing project
  const loadProject = async (): Promise<boolean> => {
    const result = await executeCommand<undefined>('load_project')
    return result !== null
  }

  // Save the current project
  const saveProject = async (): Promise<boolean> => {
    const result = await executeCommand<undefined>('save_project')
    return result !== null
  }

  // Stage management commands

  // Add a new processing stage with the given type
  const addStage = async (stage: string): Promise<boolean> => {
    const result = await executeCommand<undefined>('add_stage', { stage })
    return result !== null
  }

  // Get stage by index
  const getStage = async (index: number): Promise<ProcessingStage | null> => {
    return await executeCommand<ProcessingStage>('get_stage', { index })
  }

  // Get all stages
  const listStages = async (): Promise<ProcessingStage[]> => {
    const result = await executeCommand<ProcessingStage[]>('list_stages')
    return result || []
  }

  // Open images commands

  // Drag and drop images, function accepts paths and (optional) position to insert
  const dropImages = async (
    paths: string[],
    position?: number,
  ): Promise<boolean> => {
    const result = await executeCommand<undefined>('drop_images', { paths, position })
    return result !== null
  }

  // Open images dialog to select images, accepts (optional) position to insert
  const openImages = async (position?: number): Promise<boolean> => {
    const result = await executeCommand<undefined>('open_images', { position })
    return result !== null
  }

  // Utility commands
  //   const generateThumbnail = async (
  //     sourcePath: string,
  //     size: number,
  //   ): Promise<string | null> => {
  //     return await executeCommand<string>('generate_thumbnail_from_path', {
  //       sourcePath,
  //       size,
  //     })
  //   }

  //   const showImage = async (sourcePath: string): Promise<string | null> => {
  //     return await executeCommand<string>('show_image', { sourcePath })
  //   }

  // Get available parallelism cores
  const getCpus = async (): Promise<number> => {
    const result = await executeCommand<number>('get_cpus')
    return result || 1
  }

  // Generate a random name for a new project
  const generateRandomName = async (): Promise<string> => {
    const result = await executeCommand<string>('generate_random_name')
    return result || 'default-name'
  }

  const selectDirectory = async (): Promise<string | null> => {
    const result = await executeCommand<string>('select_directory')
    return result || null
  }

  return {
    // State
    isLoading: readonly(isLoading),
    error: readonly(error),

    // Project methods
    createProject,
    createTempProject,
    loadProject,
    saveProject,

    // Stage methods
    addStage,
    getStage,
    listStages,

    // Image methods
    dropImages,
    openImages,

    // Utility methods
    // generateThumbnail,
    // showImage,
    getCpus,
    generateRandomName,
    selectDirectory,

    // Helper methods
    clearError: () => { error.value = null },
    executeCommand, // Expose for custom commands
  }
}
