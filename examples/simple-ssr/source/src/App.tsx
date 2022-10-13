import { useState } from 'react'
import reactLogo from './assets/react.svg'
import rustLogo from './assets/rust.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <div>
        <a href="https://www.rust-lang.org" target="_blank">
          <img src={rustLogo} className="logo rust" alt="React logo" width={144} height={96}/>
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" width={144} height={96}/>
        </a>
      </div>
      <h1>Rust + React = ❤️‍🔥</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
      <p className="read-the-docs">
        Click on the logos to learn more
      </p>
    </div>
  )
}

export default App
