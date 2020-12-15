import React, { useRef, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import CircularIntegration, { ProcessTypes } from "./Components/Progress";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";
import { green } from "@material-ui/core/colors";
import VictoriaSunburst from "./Components/VictoriaSunburst";
// @ts-ignore
const rust = import('victoria-processing');

// Get the Object's methods names:
function getMethodsNames(obj: any) {
  return Object.keys(obj)
    .filter((key) => typeof obj[key] === 'function');
}

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    input: {
      display: 'none',
    },
  }),
);

function App() {
  const [processState, setProcessState] = useState(ProcessTypes.initial);
  const [wealthDistribution, setWealthDistribution] = useState<any | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const classes = useStyles();

  function onChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.item(0)
    if (file) {
      const reader = new FileReader()
      reader.onload = (readEvent) => {
        console.log("Finished reading")
        const result = reader.result as string // has to be, because we read as text
        if (result) {
          rust.then(created => {
            const save = created.process_save(result)
            setProcessState(ProcessTypes.success)
            const forex = save.js_forex_position();
            const chinese_states = forex.js_subtree_for_node(["CHI"], 1)
            setWealthDistribution(chinese_states);
          }).catch(error => {
            console.error(error)
            setProcessState(ProcessTypes.failed)
          })
        } else {
          setProcessState(ProcessTypes.failed)
        }
      }
      reader.onprogress = (progressEvent) => {
        console.log("Loaded " + progressEvent.loaded + " out of " + progressEvent.total)
      }
      reader.onerror = (progressEvent) => {
        console.log("Error loading!")
      }
      reader.readAsText(file)
    } else {
      setProcessState(ProcessTypes.cancelled)
    }
  }

  function handleClick() {
    inputRef.current?.click()
  }

  return (
    <div className="App">
      <header className="App-header">
        <p>
          Victoria econ viewer.
        </p>
        <VictoriaSunburst data={wealthDistribution}/>
        <input id="myInput"
               type="file"
               ref={inputRef}
               className={classes.input}
               onChange={onChange}
        />
        <CircularIntegration
          processState={processState}
          onClick={handleClick}
        />
      </header>
    </div>
  );
}

export default App;
