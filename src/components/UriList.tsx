import { useState, useEffect } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

import './UriList.css'
import { Textbox } from './Common/Textbox'

export function UriList() {
  const [uris, setUris] = useState([] as string[])
  const [inputValue, setInputValue] = useState<string>('')

  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as Config
      setUris(config.urls_to_redirect)
    })()
  }, [])

  const addUri = async () => {
    if (inputValue) {
      const newList = [...uris, inputValue]
      console.log("Adding: ", inputValue)
  
      setInputValue('')
      setUris(prevUris => [...prevUris, inputValue]);

      // Write to the config
      const config = await invoke('get_config') as Config
      config.urls_to_redirect = newList

      await invoke('write_config', {
        config
      })
    }
  }

  const removeUri = async (uri: string) => {
    const newList = uris.filter(u => u !== uri)
    setUris(_ => newList);

    // Write to the config
    const config = await invoke('get_config') as Config
    config.urls_to_redirect = newList

    await invoke('write_config', {
      config
    })
  }

  const handleUriChange = (uri: string, value: string) => {
    if (!value) {
      // Remove this URI from the list
      removeUri(uri)
    }
  }

  return (
    <div id="UriList">
      <span>URIs to redirect:</span>
      <div id="UriListInner">
        {/* This first textboxes content is added to the list when the user unfocusses */}
        <Textbox defaultValue={inputValue} placeholder={"Enter a new URI..."} onEnter={addUri} onBlur={addUri} onChange={setInputValue} />

        {uris.map((uri, i) => (
          <Textbox
            key={i}
            onEnter={(value) => handleUriChange(uri, value)}
            onBlur={(value) => handleUriChange(uri, value)}
            defaultValue={uri}
            onDelete={() => removeUri(uri)}
            readonly={true}
          />
        ))}
      </div>
    </div>
  )
}