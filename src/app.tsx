import { useEffect, useState } from 'preact/hooks'

import { OnOff } from './components/OnOff'
import { RedirectSelect } from './components/RedirectSelect'
import { UriList } from './components/UriList'
import { BottomControls } from './components/BottomControls'
import { TopControls } from './components/TopControls'
import { Configuration } from './components/Configuration'
import { Footer } from './components/Footer'

import './app.css'
import { invoke } from '@tauri-apps/api'

export function App() {
  const [isOn, setIsOn] = useState(false)
  const [configOpen, setConfigOpen] = useState(false)

  useEffect(() => {
    invoke('install_ca_command');
  }, [])

  return (
    <>
      <TopControls onSettingsClick={() => setConfigOpen(!configOpen)} />

      <OnOff onChange={setIsOn} />

      <RedirectSelect />

      <UriList />

      <BottomControls isOn={isOn} />

      <Footer />

      {configOpen && <Configuration onClose={() => setConfigOpen(false)} />}
    </>
  )
}
