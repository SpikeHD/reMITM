import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'preact/hooks'

import './Configuration.css'

import CloseButton from '../assets/close.svg'
import { Checkbox } from './Common/Checkbox'
import { Textbox } from './Common/Textbox'
import { DirTextbox } from './Common/DirTextbox'

interface Props {
  onClose: () => void
}

interface PartialConfig {
  launch_at_startup: boolean
  proxy_port: number
  terminal: string
  modify_gsettings: boolean
  use_env_variables: boolean
}

export function Configuration(props: Props) {
  const [config, setConfig] = useState<PartialConfig>({
    launch_at_startup: false,
    proxy_port: 0,
    terminal: '',
    modify_gsettings: false,
    use_env_variables: false,
  })
  const [hide, setHide] = useState(true)
  const [isLinux, setIsLinux] = useState(false)

  useEffect(() => {
    ;(async () => {
      setHide(false)
      setConfig((await invoke('get_config')) as PartialConfig)
      setIsLinux((await invoke('get_platform')) === 'linux')
    })()
  }, [])

  async function setConfigValue<K extends keyof Config>(
    key: K,
    value: Config[K]
  ) {
    const config = (await invoke('get_config')) as Config
    config[key] = value

    await invoke('write_config', {
      config,
    })
  }

  const setStartup = async (value: boolean) => {
    await setConfigValue('launch_at_startup', value)
  }

  const setProxyPort = async (value: string) => {
    await setConfigValue('proxy_port', parseInt(value))
  }

  const setTerminal = async (value: string) => {
    await setConfigValue('terminal', value)
  }

  const setGsettings = async (value: boolean) => {
    await setConfigValue('modify_gsettings', value)
  }

  const setEnvVariables = async (value: boolean) => {
    await setConfigValue('use_env_variables', value)
  }

  return (
    <div id="Configuration" className={hide ? 'hide' : ''}>
      <div id="ConfigurationTop">
        <img src={CloseButton} onClick={props.onClose} />
      </div>

      <div id="ConfigurationInner">
        <div className="ConfigurationRow">
          <div className="ConfigurationText">Launch on Startup</div>
          <div className="ConfigurationControl">
            <Checkbox
              defaultValue={config?.launch_at_startup}
              onChange={setStartup}
            />
          </div>
        </div>

        <div className="ConfigurationRow">
          <div className="ConfigurationText">Proxy Port</div>
          <div className="ConfigurationControl PortConfig">
            <Textbox
              defaultValue={config?.proxy_port.toString()}
              onBlur={setProxyPort}
              onEnter={setProxyPort}
            />
          </div>
        </div>

        <div className="ConfigurationRow">
          <div className="ConfigurationText">Terminal</div>
          <div className="ConfigurationControl">
            <DirTextbox
              defaultValue={config?.terminal}
              onChange={setTerminal}
            />
          </div>
        </div>

        <div className={'ConfigurationRow ' + (!isLinux && 'disabled')}>
          <div className="ConfigurationText">Modify GSettings</div>
          <div className="ConfigurationControl">
            <Checkbox
              disabled={!isLinux}
              defaultValue={config?.modify_gsettings}
              onChange={setGsettings}
            />
          </div>
        </div>

        <div className={'ConfigurationRow ' + (!isLinux && 'disabled')}>
          <div className="ConfigurationText">Use Environment Variables</div>
          <div className="ConfigurationControl">
            <Checkbox
              disabled={!isLinux}
              defaultValue={config?.use_env_variables}
              onChange={setEnvVariables}
            />
          </div>
        </div>
      </div>
    </div>
  )
}
