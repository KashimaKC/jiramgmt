'use client'

import { home } from "@/styles/styles"

import { invoke } from "@tauri-apps/api/tauri"

import Timer from "@/components/timer"
import SysUsage from "@/components/sysUsage"
import { useEffect, useState } from "react"

const Home = () => {

  const [logs, setLogs] = useState<any>()

  const invokeCommand = async () => {
    let date = new Date;
    await invoke('insert_db', { 'date': date.toLocaleString(), 'task': 'test' })
  }
  
  useEffect(() => {
    const retrieveLogs = async () => {
      let logs = await invoke('retrieve_logs')
      setLogs(logs)
    }

    retrieveLogs()
  }, [])


  return (
    <main style={home.homeContainer as React.CSSProperties}>
      <button onClick={invokeCommand}>DB Insert</button>
      { logs }
      <Timer />
      <SysUsage />
    </main>
  );
}

export default Home
