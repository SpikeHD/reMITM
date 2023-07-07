import { useEffect, useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api'

interface Props {
  text: string
}

export function Tr(props: Props) {
  const [text, setText] = useState('')

  useEffect(() => {
    ;(async () => {
      setText(await tr(props.text))
    })()
  }, [text])

  return <>{text}</>
}

export async function tr(key: string) {
  const lang = JSON.parse(await invoke('get_language'))
  const enFallback = JSON.parse(
    await invoke('get_language', {
      lang: 'en',
    })
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  ) as Record<string, any>
  const keyTree = key.split('.')

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const findResult = (obj: Record<string, any>) => {
    let result

    for (let i = 0; i < keyTree.length; i++) {
      const key = keyTree[i]

      if (!result) {
        result = obj[key]
      } else {
        result = result[key]
      }
    }

    return result
  }

  const result =
    findResult(lang) || findResult(enFallback) || 'MISSING TRANSLATION'

  return result
}
