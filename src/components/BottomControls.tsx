import { invoke } from '@tauri-apps/api'

import './BottomControls.css'

import TerminalIcon from '../assets/terminal.svg'
import LogIcon from '../assets/text.svg'

interface Props {
  isOn: boolean
}

export function BottomControls(props: Props) {
  const openShell = async () => {
    await invoke('open_shell')
  }

  const openLogs = async () => {
    await invoke('open_log_window')
  }

  return (
    <div id="BottomControls" className={props.isOn ? '' : 'hide'}>
      <span>
        <img src={TerminalIcon} onClick={openShell} />
      </span>
      <span>
        <img src={LogIcon} onClick={openLogs} />
      </span>
    </div>
  )
}
