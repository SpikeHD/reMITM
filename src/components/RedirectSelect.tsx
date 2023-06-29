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
      
      // Split the rest of the url from the http(s):// part
      const split = config.redirect_to.split('://')
      const redirect = split[1] || config.redirect_to

      // Split url and port
      const url = redirect.split(':')
      const port = url[1]
      const urlWithoutPort = url[0]

      setRedirect(urlWithoutPort)
      setPort(port)

      // Set the redirect server
      await invoke('set_redirect_server', {
        server: `${urlWithoutPort}:${port}`
      })
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