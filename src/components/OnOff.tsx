import { useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'
//import { listen } from '@tauri-apps/api/event'

import './OnOff.css'
import OnOffSVG from '../assets/onoff.svg'

interface Props {
  onChange?: (value: boolean) => void
}

export function OnOff(props: Props) {
  const [isOn, setIsOn] = useState(false)
  const [connectState, setConnectState] = useState('Disconnected')

  const toggle = async () => {
    setIsOn(!isOn)

    setConnectState(isOn ? 'Disconnected' : 'Connecting...')

    isOn ? await invoke('disconnect') : await invoke('connect')

    if (props.onChange) {
      props.onChange(!isOn)
    }

    setConnectState(isOn ? 'Disconnected' : 'Connected')
  }

  return (
    <div id="OnOffToggle" onClick={toggle}>
      <span>{connectState}</span>
      <img src={OnOffSVG} className={isOn ? 'toggled' : ''} />
    </div>
  )
}
