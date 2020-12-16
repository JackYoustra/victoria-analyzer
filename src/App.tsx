import React, { useRef, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import CircularIntegration, { ProcessTypes } from "./Components/Progress";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";
import { green } from "@material-ui/core/colors";
import VictoriaSunburst from "./Components/VictoriaSunburst";
import { D3Node } from "victoria-processing";
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
  const [wealthDistribution, setWealthDistribution] = useState<D3Node | undefined>(undefined);
  const [topLabel, setTopLabel] = useState<string>("Victoria econ viewer")
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
            setTopLabel("Reading and processing " + file.name);
            const save = created.process_save(result)
            setProcessState(ProcessTypes.success)
            const forex = save.js_forex_position();
            setWealthDistribution(forex);
            setTopLabel("Displaying " + file.name);
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
          {topLabel}
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
