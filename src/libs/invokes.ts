import { invoke } from '@tauri-apps/api'

export async function print(s: string): Promise<void> {
  return await invoke('print', { s })
}

export async function downloadChromedriverVersions(): Promise<string> {
  return await invoke('download_chromedriver_versions', {})
}

export async function downloadChromedriverKnownVersions(): Promise<string> {
  return await invoke('download_chromedriver_known_versions', {})
}
