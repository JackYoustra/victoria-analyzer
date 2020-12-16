import React, { MouseEventHandler, useCallback, useEffect, useRef, useState } from 'react';
import { NormalizedDatum, ResponsiveSunburst } from '@nivo/sunburst'
import { useTheme } from '@nivo/core'
import { D3Node } from "victoria-processing";
import Button from "@material-ui/core/Button";

interface SunburstProps {
  data?: D3Node,
}

// https://stackoverflow.com/a/16233919/998335
const formatter = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',

  // These options are needed to round to whole numbers if that's what you want.
  //minimumFractionDigits: 0, // (this suffices for whole numbers, but will print 2500.10 as $2,500.1)
  //maximumFractionDigits: 0, // (causes 2500.99 to be printed as $2,501)
});

const CustomTooltip = ({ data, value, color }: NormalizedDatum<unknown>) => {
  const theme = useTheme()
  return (
    <strong style={{ ...theme.tooltip.container, color }}>
      {
        // @ts-ignore
        data.name
      }: {formatter.format(value)}
    </strong>
  )
}

// @ts-ignore
const flatten = (data: any[]) =>
  data.reduce((acc, item) => {
    if (item.children) {
      return [...acc, item, ...flatten(item.children)]
    }

    return [...acc, item]
  }, [])

const findObject = (data: any[], id: React.ReactText) => data.find(searchedName => searchedName.id === id)

export default function VictoriaSunburst(props: SunburstProps) {
  const [chartData, setChartData] = useState<any | null>(null);
  const [pieStack, setPieStack] = useState<any[]>([]);

  useEffect(() => {
    const chart_data = props.data?.js_subtree_for_node([], 500);
    // console.log("Set chart data to " + JSON.stringify(chart_data));
    setChartData(chart_data);
    setPieStack([chart_data]);
  }, [props.data])

  if (!chartData) {
    return (<></>)
  }
  return (
    <div style={{ height: '600px', width: '500px' }}>
      Money Chart - Where is all the money in the game?
      <Button
        variant="contained"
        color="primary"
        disabled={pieStack.length <= 1}
        onClick={() => {
          setPieStack(pieStack.slice(0, pieStack.length - 1));
          setChartData(pieStack[pieStack.length - 2]);
        }}
      >
        Up a level
      </Button>
      <ResponsiveSunburst
        data={chartData}
        margin={{ top: 40, right: 20, bottom: 20, left: 20 }}
        id="id"
        value="size"
        cornerRadius={2}
        borderWidth={1}
        borderColor="white"
        colors={{ scheme: 'nivo' }}
        childColor={{ from: 'color' }}
        animate={true}
        motionConfig="gentle"
        isInteractive={true}
        // valueFormat=" >-$,.2f"
        tooltip={CustomTooltip}
        theme={{
          tooltip: {
            container: {
              background: '#333',
            },
          },
        }}
        onClick={clickedData => {
          const foundObject = findObject(flatten(chartData.children), clickedData.id)
          if (foundObject && foundObject.children) {
            console.log("Zoom chart data to " + JSON.stringify(foundObject));
            setPieStack([...pieStack, foundObject]);
            setChartData(foundObject);
          }
        }}
        enableSliceLabels={true}
      />
    </div>
  );
}