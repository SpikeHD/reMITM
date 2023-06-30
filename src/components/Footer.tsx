import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api'
import { useEffect, useState } from 'preact/hooks'

import './Footer.css'

export function Footer() {
  const [version, setVersion] = useState('1.0.0')
  const [hash, setHash] = useState('UNKNOWN')

  useEffect(() => {
    ;(async () => {
      setVersion(await getVersion())
      setHash(await invoke('get_hash'))
    })()
  }, [version])

  return (
    <div id="Footer">
      reMITM - {version} | Commit hash: {hash}
    </div>
  )
}
