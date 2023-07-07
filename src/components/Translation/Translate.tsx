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

  return (
    <>
      {text}
    </>
  )
}

export async function tr(key: string) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const lang = JSON.parse(await invoke('get_language')) as Record<string, any>
  const keyTree = key.split('.')

  let result

  for (let i = 0; i < keyTree.length; i++) {
    const key = keyTree[i]

    if (!result) {
      result = lang[key]
    } else {
      result = result[key]
    }
  }

  return result
}