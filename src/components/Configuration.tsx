import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'preact/hooks'

import './Configuration.css'

import CloseButton from '../assets/close.svg'

interface Props {
  onClose: () => void
}

export function Configuration(props: Props) {
  const [config, setConfig] = useState<Config>()
  const [hide, setHide] = useState(true)
  
  useEffect(() => {
    (async () => {
      const config = await invoke('get_config') as Config
      setConfig(config)

      setHide(false)
    })()
  }, [])

  return (
    <div id="Configuration" class={hide ? 'hide' : ''}>
      <div id="ConfigurationTop">
        <img src={CloseButton} onClick={props.onClose} />
      </div>

      <div id="ConfigurationInner">
        <div class="ConfigurationRow">
          <div class="ConfigurationText"></div>
        </div>
      </div>
    </div>
  )
}