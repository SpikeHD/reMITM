import { open } from '@tauri-apps/api/dialog'
import { useState } from 'preact/hooks'
import { Textbox } from './Textbox'

import './DirTextbox.css'
import File from '../../assets/folder.svg'

interface Props {
  onChange?: (value: string) => void
  onBlur?: (value: string) => void
  onEnter?: (value: string) => void
  onDelete?: () => void
  defaultValue?: string
  placeholder?: string
  readonly?: boolean
  class?: string

  folder?: boolean
  extensions?: string[]
}

export function DirTextbox(props: Props) {
  const [value, setValue] = useState('')

  const handleIconClick = async () => {
    let path

    if (props.folder) {
      path = await open({
        directory: true,
      })
    } else {
      path = await open({
        filters: [{ name: 'Files', extensions: props.extensions || ['*'] }],
      })
    }

    if (Array.isArray(path)) path = path[0]
    if (!path) return

    setValue(path)

    if (props.onChange) props.onChange(path)
  }

  return (
    <div className="DirTextbox">
      <Textbox
        {...props}
        class={props.class + ' DirTextbox'}
        placeholder={props.placeholder || 'Enter a new directory...'}
        value={value}
      />

      <div className="DirTextboxButton" onClick={handleIconClick}>
        <img src={File} />
      </div>
    </div>
  )
}
