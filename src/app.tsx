import { useState } from 'preact/hooks'

import { OnOff } from './components/OnOff'
import { RedirectSelect } from './components/RedirectSelect'
import { UriList } from './components/UriList'

import './app.css'
import { BottomControls } from './components/BottomControls'
import { TopControls } from './components/TopControls'
import { Configuration } from './components/Configuration'

export function App() {
  const [isOn , setIsOn ] = useState(false)
  const [configOpen, setConfigOpen] = useState(false)

  return (
    <>
      <TopControls onSettingsClick={() => setConfigOpen(!configOpen)}/>

      <OnOff onChange={setIsOn} />

      <RedirectSelect />

      <UriList />

      <BottomControls isOn={isOn} />

      {
        configOpen && (
          <Configuration onClose={() => setConfigOpen(false)} />
        )
      }
    </>
  )
}
