import { useState, useEffect } from 'preact/compat'
import { Textbox } from './Textbox'
import { invoke } from '@tauri-apps/api'

import './RedirectSelect.css'

export function RedirectSelect() {
  const [redirect, setRedirect] = useState('')
  const [port, setPort] = useState('')

  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as Config
      setRedirect(config.redirect_to)
    })()
  }, [])

  const handleRedirectChange = async (value: string) => {
    await setRedirect(value)
    handleChange()
  }

  const handlePortChange = async (value: string) => {
    await setPort(value)
    handleChange()
  }

  const handleChange = async () => {
    // Write to the config
    const config = await invoke('get_config') as Config
    config.redirect_to = `${redirect}:${port}`

    await invoke('write_config', {
      config
    })

    // Change redirect_to internally
    await invoke('set_redirect_server', {
      server: config.redirect_to
    })
  }

  return (
    <div id="RedirectSelect">
      <span>Redirect to:</span>
      <div class="RedirectInner">
        <Textbox placeholder='Server address...' class='RedirectServer' defaultValue={redirect} onBlur={handleRedirectChange} onEnter={handleRedirectChange} />
        <Textbox placeholder='Port...' class='RedirectPort' defaultValue={port} onBlur={handlePortChange} onEnter={handlePortChange} />
      </div>
    </div>
  )
}