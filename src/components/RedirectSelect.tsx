import { useState, useEffect } from 'preact/compat'
import { Textbox } from './Textbox'
import { invoke } from '@tauri-apps/api'

import './RedirectSelect.css'

export function RedirectSelect() {
  const [redirect, setRedirect] = useState('' as string)

  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as Config
      setRedirect(config.redirect_to)
    })()
  }, [])

  const handleRedirectChange = async (value: string) => {
    setRedirect(value)

    // Write to the config
    const config = await invoke('get_config') as Config
    config.redirect_to = value

    await invoke('write_config', {
      config
    })
  }

  return (
    <div id="RedirectSelect">
      <span>Redirect to:</span>
      <Textbox defaultValue={redirect} onBlur={handleRedirectChange} onEnter={handleRedirectChange} />
    </div>
  )
}