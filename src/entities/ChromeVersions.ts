export type Channel = 'Stable' | 'Beta' | 'Dev' | 'Canary'
export type Binary = 'chrome' | 'chromedriver' | 'chrome-headless-shell'

export type ChromeVersionsWithDwonloadsDownloads = {
  platform: string
  url: string
}[]

export type ChromeVersionsWithDwonloadsVersion = {
  version: string
  revision: string
  downloads: { [binary in Binary]: ChromeVersionsWithDwonloadsDownloads }
}

export type ChromeVersionsWithDwonloads = {
  timestamp: string
  versions: ChromeVersionsWithDwonloadsVersion[]
}

export type ChromeKnownVersionsChannel = {
  // Stable, Beta, Dev, Canary
  channel: Channel
  version: string
  revision: string
}

export type ChromeKnownVersions = {
  timestamp: string
  channels: {
    [channel in Channel]: ChromeKnownVersionsChannel
  }
}
