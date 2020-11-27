import React from 'react';
import logo from './logo.svg';
import './App.css';
// @ts-ignore
const rust = import('victoria-processing');

// Get the Object's methods names:
function getMethodsNames(obj: any) {
  return Object.keys(obj)
      .filter((key) => typeof obj[key] === 'function');
}

function pdx2json(text: string): object {
  rust.then(created => {
    console.log(getMethodsNames(created))
    created.greet()
  }).catch(console.error)
  return text as unknown as object
}

function App() {
  function onChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.item(0)
    if (file) {
      const reader = new FileReader()
      reader.onload = (readEvent) => {
        console.log("Finished reading")
        const result = reader.result as string // has to be, because we read as text
        if (result) {
          pdx2json(result)
        }
      }
      reader.onprogress = (progressEvent) => {
        console.log("Loaded " + progressEvent.loaded + " out of " + progressEvent.total)
      }
      reader.onerror = (progressEvent) => {
        console.log("Error loading!")
      }
      reader.readAsText(file)
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>
          Victoria econ viewer.
        </p>
        <input id="myInput"
          type="file"
          onChange={onChange}
        />
      </header>
    </div>
  );
}

export default App;
