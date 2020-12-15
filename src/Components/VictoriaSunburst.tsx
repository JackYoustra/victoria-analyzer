import React, { useRef, useState } from 'react';
import { NormalizedDatum, ResponsiveSunburst } from '@nivo/sunburst'
import { useTheme } from '@nivo/core'

interface SunburstProps {
  data?: any,
}

// https://stackoverflow.com/a/16233919/998335
const formatter = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',

  // These options are needed to round to whole numbers if that's what you want.
  //minimumFractionDigits: 0, // (this suffices for whole numbers, but will print 2500.10 as $2,500.1)
  //maximumFractionDigits: 0, // (causes 2500.99 to be printed as $2,501)
});

const CustomTooltip = ({ id, value, color }: NormalizedDatum<unknown>) => {
  const theme = useTheme()

  return (
    <strong style={{ ...theme.tooltip.container, color }}>
      {id}: {formatter.format(value)}
    </strong>
  )
}

export default function VictoriaSunburst(props: SunburstProps) {
  if (!props.data) {
    return (<></>)
  }
  return (
    <div style={{ height: '500px', width: '500px' }}>
      <ResponsiveSunburst
        data={props.data}
        // height={500}
        // width={500}
        // getLabel={(d: { name: any; }) => d.name}
        // getSize={(d: { size: any; }) => d.size}
        // padAngle={() => 0.02}
        // getColor={() => pallette.next()}
        margin={{ top: 40, right: 20, bottom: 20, left: 20 }}
        id="name"
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
      />
    </div>
  );
}