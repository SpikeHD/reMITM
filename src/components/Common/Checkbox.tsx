import { useEffect, useState } from 'preact/hooks'

import './Checkbox.css'

import Checkmark from '../../assets/check.svg'

interface Props {
  onChange?: (value: boolean) => void
  defaultValue?: boolean
  disabled?: boolean
}

export function Checkbox(props: Props) {
  const [checked, setChecked] = useState(false)

  useEffect(() => {
    setChecked(props.defaultValue || false)

    console.log('Is checked? ', props.defaultValue)
  }, [props.defaultValue])

  return (
    <div
      className="CheckboxOuter"
      onClick={() => {
        if (props.disabled) return

        const newValue = !checked
        setChecked(newValue)
        props.onChange?.(newValue)
      }}
    >
      <div className="Checkbox">{checked && <img src={Checkmark} />}</div>
    </div>
  )
}
