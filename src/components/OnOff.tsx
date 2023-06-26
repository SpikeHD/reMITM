import { useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

import './OnOff.css'
import OnOffSVG from '../assets/onoff.svg'

export function OnOff() {
  const [isOn, setIsOn] = useState(false)

  const toggle = () => {
    setIsOn(!isOn)

    isOn ? invoke('disconnect_from_proxy') : invoke('connect_to_proxy')
  }

  return (
    <div id="OnOffToggle" onClick={toggle}>
    <span>{isOn ? 'Connected' : 'Disconnected'}</span>
      <img src={OnOffSVG} class={isOn ? 'toggled' : ''} />
    </div>
  )
}