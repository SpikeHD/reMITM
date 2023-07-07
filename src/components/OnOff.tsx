import { useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/tauri'

import './OnOff.css'
import OnOffSVG from '../assets/onoff.svg'
import { tr } from './Translation/Translate'

interface Props {
  onChange?: (value: boolean) => void
}

export function OnOff(props: Props) {
  const [isOn, setIsOn] = useState(false)
  const [connectState, setConnectState] = useState('Disconnected')

  const toggle = async () => {
    setIsOn(!isOn)

    setConnectState(isOn ? await tr('main.disconnected') : await tr('main.connecting'))

    isOn ? await invoke('disconnect') : await invoke('connect')

    if (props.onChange) {
      props.onChange(!isOn)
    }

    setConnectState(isOn ? await tr('main.disconnected') : await tr('main.connected'))
  }

  return (
    <div id="OnOffToggle" onClick={toggle}>
      <span>{connectState}</span>
      <img src={OnOffSVG} className={isOn ? 'toggled' : ''} />
    </div>
  )
}
