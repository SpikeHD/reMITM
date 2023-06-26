import { useState, useEffect } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

import './UriList.css'

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
      <button onClick={addUri}>Add URI</button>
      <ul>
        {uris.map(uri => (
          <li>
            <span>{uri}</span>
            <button onClick={() => removeUri(uri)}>Remove</button>
          </li>
        ))}
      </ul>
    </div>
  )
}