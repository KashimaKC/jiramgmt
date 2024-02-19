'use client'

import { invoke } from "@tauri-apps/api/tauri"
import { listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react"

const SysUsage = () => {
    const [cpu, setCpu] = useState<any>()

    const invokeCommand = async () => {
      setCpu(await invoke('get_cpu_usage'))
    }
  
    useEffect(() => {
  
      invokeCommand()
  
      listen(
        "cpu_usage", (e: any) => {
          setCpu(e.payload.value)
        }
      )
    }, [])

    return (
        <div>
            Cpu Usage: { Math.round(cpu) }%
        </div>
    )
}

export default SysUsage

