'use client'

import { invoke } from "@tauri-apps/api/tauri"
import { listen } from "@tauri-apps/api/event"
import { useEffect, useState } from "react"


const Timer = () => {
    
    const [time, setTime] = useState<any>()

    const invokeCommand = async () => {
        setTime(await invoke('counter'))
      }

    useEffect(() => {

        invokeCommand()

        listen(
            "count", (e: any) => {
              setTime(e.payload.value)
            }
          )
    }, [])

    let elapsedTime = new Date(time * 1000)

    return (
        <div>
            Time Elapsed: { 
                elapsedTime.getUTCHours().toString().padStart(2, '0')}:{ 
                elapsedTime.getUTCMinutes().toString().padStart(2, '0') }:{ 
                elapsedTime.getUTCSeconds().toString().padStart(2, '0') }
        </div>
    )
}

export default Timer