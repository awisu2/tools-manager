<script lang="ts">
  import { onMount } from 'svelte'
  import type {
    ChromeKnownVersions,
    ChromeVersionsWithDwonloads,
    ChromeVersionsWithDwonloadsVersion
  } from '../entities/ChromeVersions'
  import {
    downloadChromedriverVersions,
    downloadChromedriverKnownVersions
  } from '../libs/invokes'

  let data: ChromeVersionsWithDwonloads | null = null
  let knownVersions: ChromeKnownVersions | null = null

  async function load() {
    let s = await downloadChromedriverVersions()
    data = JSON.parse(s)

    s = await downloadChromedriverKnownVersions()
    knownVersions = JSON.parse(s)
  }

  function getVersion(version: string): ChromeVersionsWithDwonloadsVersion | undefined {
    if (!data?.versions) {
      return undefined
    }
    let _version = data?.versions.find((v) => v.version == version)
    return _version
  }

  onMount(() => {
    load()
  })
</script>

<div>
  chrome driver
  <button on:click={load}>roload</button>
  <div>orign: https://googlechromelabs.github.io/chrome-for-testing/</div>

  {#if knownVersions}
    <ul class="ul ul-indent">
      {#each Object.entries(knownVersions.channels) as [k, channel]}
        {#if k == 'Stable'}
          <li>
            [{channel.channel}]: {channel.version}
            <ul class="ul ul-indent">
              {#each Object.entries(getVersion(channel.version)?.downloads || {}) as [binary, download]}
                {#if binary == 'chromedriver'}
                  <li>
                    {binary}
                    <ul class="ul ul-indent">
                      {#each download as d}
                        <li>{d.platform}: <a href={d.url}>{d.url}</a></li>
                      {/each}
                    </ul>
                  </li>
                {/if}
              {/each}
            </ul>
          </li>
        {/if}
      {/each}
    </ul>
  {/if}

  {#if data}
    <details>
      <summary class="pointer">infos</summary>
      <ul class="ul">
        <li>timestamp: {data.timestamp}</li>

        {#each data.versions as version}
          <li>
            {version.version},
            {version.revision},
            <ul class="ul ul-indent">
              {#each Object.keys(version.downloads) as binaly}
                <li>
                  <div>{binaly}</div>
                  <ul class="ul ul-indent">
                    {#each version.downloads[binaly] as download}
                      <li>
                        {download.platform}, <a href={download.url}>{download.url}</a>
                      </li>
                    {/each}
                  </ul>
                </li>
              {/each}
            </ul>
          </li>
        {/each}
      </ul>
    </details>
  {/if}
</div>
