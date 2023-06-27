import { useState, useEffect } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

import './UriList.css'
import { Textbox } from './Textbox'

export function UriList() {
  const [uris, setUris] = useState([] as string[])

  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as Config
      setUris(config.urls_to_redirect)
    })()
  })

  const addUri = () => {
    const uri = prompt('Enter a URI')
    if (uri) {
      setUris([...uris, uri])
    }
  }

  const removeUri = (uri: string) => {
    setUris(uris.filter(u => u !== uri))
  }

  return (
    <div id="UriList">
      <span>URIs to redirect:</span>
      <div id="UriListInner">
        {uris.map(uri => (
          <Textbox value={uri} />
        ))}
      </div>
    </div>
  )
}