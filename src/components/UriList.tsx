import { useState, useEffect } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

import { Textbox } from './Common/Textbox'
import { Tr, tr } from './Translation/Translate'

import './UriList.css'
import File from '../assets/doc.svg'

export function UriList() {
  const [uris, setUris] = useState([] as string[])
  const [inputValue, setInputValue] = useState<string>('')
  const [placeholder, setPlaceholder] = useState<string>('')

  useEffect(() => {
    ;(async () => {
      const config = (await invoke('get_config')) as Config
      setUris(config.urls_to_redirect)

      setPlaceholder(await tr('main.enter_new_uri'))
    })()
  }, [])

  const addUri = async (uri?: string) => {
    const value = uri || inputValue

    if (value) {
      const newList = [...uris, value]
      console.log('Adding: ', value)

      setInputValue('')
      setUris((prevUris) => [...prevUris, value])

      // Write to the config
      const config = (await invoke('get_config')) as Config
      config.urls_to_redirect = newList

      await invoke('write_config', {
        config,
      })
    }
  }

  const removeUri = async (uri: string) => {
    const newList = uris.filter((u) => u !== uri)
    setUris((_) => newList)

    // Write to the config
    const config = (await invoke('get_config')) as Config
    config.urls_to_redirect = newList

    await invoke('write_config', {
      config,
    })
  }

  const handleUriChange = (uri: string, value: string) => {
    if (!value) {
      // Remove this URI from the list
      removeUri(uri)
    }
  }

  const handleSelectFile = async () => {
    // Open file select dialog
    const file = await dialog.open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'Text Files',
          extensions: ['txt'],
        },
      ],
    })

    if (!file) return

    const fileContents: string = await invoke('read_as_text', {
      path: file,
    })

    if (!fileContents) return

    fileContents.split('\n').forEach((uri: string) => {
      if (!uri) return
      addUri(uri)
    })
  }

  return (
    <div id="UriList">
      <span>
        <Tr text={'main.uris_to_redirect'} />
      </span>
      <div id="UriListInner">
        {/* This first textboxes content is added to the list when the user unfocusses */}
        <div id="UriListTextboxContainer">
          <Textbox
            defaultValue={inputValue}
            placeholder={placeholder}
            onEnter={addUri}
            onBlur={addUri}
            onChange={setInputValue}
          />

          <div id="UriListFile" onClick={handleSelectFile}>
            <img src={File} />
          </div>
        </div>

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
