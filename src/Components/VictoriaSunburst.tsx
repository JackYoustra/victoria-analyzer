import React, { useRef, useState } from 'react';
import { ResponsiveSunburst } from '@nivo/sunburst';

interface SunburstProps {
  data?: any,
}

const uwu = String.raw`{
  "name": "nivo",
  "color": "hsl(128, 70%, 50%)",
  "children": [
    {
      "name": "viz",
      "label": "uwu moment",
      "color": "hsl(201, 70%, 50%)",
      "children": [
        {
          "name": "stack",
          "color": "hsl(74, 70%, 50%)",
          "children": [
            {
              "name": "cchart",
              "color": "hsl(129, 70%, 50%)",
              "loc": 139671
            },
            {
              "name": "xAxis",
              "color": "hsl(198, 70%, 50%)",
              "loc": 193816
            },
            {
              "name": "yAxis",
              "color": "hsl(137, 70%, 50%)",
              "loc": 178270
            },
            {
              "name": "layers",
              "color": "hsl(117, 70%, 50%)",
              "loc": 11785
            }
          ]
        },
        {
          "name": "ppie",
          "color": "hsl(31, 70%, 50%)",
          "children": [
            {
              "name": "chart",
              "color": "hsl(269, 70%, 50%)",
              "children": [
                {
                  "name": "pie",
                  "color": "hsl(294, 70%, 50%)",
                  "children": [
                    {
                      "name": "outline",
                      "color": "hsl(354, 70%, 50%)",
                      "loc": 71353
                    },
                    {
                      "name": "slices",
                      "color": "hsl(157, 70%, 50%)",
                      "loc": 63196
                    },
                    {
                      "name": "bbox",
                      "color": "hsl(227, 70%, 50%)",
                      "loc": 129208
                    }
                  ]
                },
                {
                  "name": "donut",
                  "color": "hsl(42, 70%, 50%)",
                  "loc": 57274
                },
                {
                  "name": "gauge",
                  "color": "hsl(7, 70%, 50%)",
                  "loc": 108906
                }
              ]
            },
            {
              "name": "legends",
              "color": "hsl(21, 70%, 50%)",
              "loc": 75744
            }
          ]
        }
      ]
    },
    {
      "name": "colors",
      "color": "hsl(105, 70%, 50%)",
      "children": [
        {
          "name": "rgb",
          "color": "hsl(228, 70%, 50%)",
          "loc": 80943
        },
        {
          "name": "hsl",
          "color": "hsl(221, 70%, 50%)",
          "loc": 139188
        }
      ]
    },
    {
      "name": "utils",
      "color": "hsl(214, 70%, 50%)",
      "children": [
        {
          "name": "randomize",
          "color": "hsl(77, 70%, 50%)",
          "loc": 39242
        },
        {
          "name": "resetClock",
          "color": "hsl(172, 70%, 50%)",
          "loc": 110546
        },
        {
          "name": "noop",
          "color": "hsl(78, 70%, 50%)",
          "loc": 78336
        },
        {
          "name": "tick",
          "color": "hsl(265, 70%, 50%)",
          "loc": 152233
        },
        {
          "name": "forceGC",
          "color": "hsl(328, 70%, 50%)",
          "loc": 91933
        },
        {
          "name": "stackTrace",
          "color": "hsl(102, 70%, 50%)",
          "loc": 131837
        },
        {
          "name": "dbg",
          "color": "hsl(218, 70%, 50%)",
          "loc": 147865
        }
      ]
    },
    {
      "name": "generators",
      "color": "hsl(357, 70%, 50%)",
      "children": [
        {
          "name": "address",
          "color": "hsl(337, 70%, 50%)",
          "loc": 89689
        },
        {
          "name": "city",
          "color": "hsl(94, 70%, 50%)",
          "loc": 27595
        },
        {
          "name": "animal",
          "color": "hsl(219, 70%, 50%)",
          "loc": 79458
        },
        {
          "name": "movie",
          "color": "hsl(134, 70%, 50%)",
          "loc": 61093
        },
        {
          "name": "user",
          "color": "hsl(140, 70%, 50%)",
          "loc": 42105
        }
      ]
    },
    {
      "name": "set",
      "color": "hsl(51, 70%, 50%)",
      "children": [
        {
          "name": "clone",
          "color": "hsl(22, 70%, 50%)",
          "loc": 88080
        },
        {
          "name": "intersect",
          "color": "hsl(307, 70%, 50%)",
          "loc": 15573
        },
        {
          "name": "merge",
          "color": "hsl(228, 70%, 50%)",
          "loc": 176077
        },
        {
          "name": "reverse",
          "color": "hsl(44, 70%, 50%)",
          "loc": 31029
        },
        {
          "name": "toArray",
          "color": "hsl(60, 70%, 50%)",
          "loc": 110640
        },
        {
          "name": "toObject",
          "color": "hsl(109, 70%, 50%)",
          "loc": 10039
        },
        {
          "name": "fromCSV",
          "color": "hsl(347, 70%, 50%)",
          "loc": 30538
        },
        {
          "name": "slice",
          "color": "hsl(91, 70%, 50%)",
          "loc": 87246
        },
        {
          "name": "append",
          "color": "hsl(196, 70%, 50%)",
          "loc": 58919
        },
        {
          "name": "prepend",
          "color": "hsl(246, 70%, 50%)",
          "loc": 96246
        },
        {
          "name": "shuffle",
          "color": "hsl(250, 70%, 50%)",
          "loc": 15121
        },
        {
          "name": "pick",
          "color": "hsl(232, 70%, 50%)",
          "loc": 32108
        },
        {
          "name": "plouc",
          "color": "hsl(317, 70%, 50%)",
          "loc": 101719
        }
      ]
    },
    {
      "name": "text",
      "color": "hsl(265, 70%, 50%)",
      "children": [
        {
          "name": "trim",
          "color": "hsl(294, 70%, 50%)",
          "loc": 105456
        },
        {
          "name": "slugify",
          "color": "hsl(5, 70%, 50%)",
          "loc": 246
        },
        {
          "name": "snakeCase",
          "color": "hsl(6, 70%, 50%)",
          "loc": 73986
        },
        {
          "name": "camelCase",
          "color": "hsl(101, 70%, 50%)",
          "loc": 156817
        },
        {
          "name": "repeat",
          "color": "hsl(55, 70%, 50%)",
          "loc": 35068
        },
        {
          "name": "padLeft",
          "color": "hsl(147, 70%, 50%)",
          "loc": 15809
        },
        {
          "name": "padRight",
          "color": "hsl(91, 70%, 50%)",
          "loc": 8179
        },
        {
          "name": "sanitize",
          "color": "hsl(16, 70%, 50%)",
          "loc": 30568
        },
        {
          "name": "ploucify",
          "color": "hsl(32, 70%, 50%)",
          "loc": 157477
        }
      ]
    },
    {
      "name": "misc",
      "color": "hsl(25, 70%, 50%)",
      "children": [
        {
          "name": "greetings",
          "color": "hsl(40, 70%, 50%)",
          "children": [
            {
              "name": "hey",
              "color": "hsl(320, 70%, 50%)",
              "loc": 8015
            },
            {
              "name": "HOWDY",
              "color": "hsl(217, 70%, 50%)",
              "loc": 146388
            },
            {
              "name": "aloha",
              "color": "hsl(195, 70%, 50%)",
              "loc": 24392
            },
            {
              "name": "AHOY",
              "color": "hsl(311, 70%, 50%)",
              "loc": 70578
            }
          ]
        },
        {
          "name": "other",
          "color": "hsl(205, 70%, 50%)",
          "loc": 101477
        },
        {
          "name": "path",
          "color": "hsl(31, 70%, 50%)",
          "children": [
            {
              "name": "pathA",
              "color": "hsl(178, 70%, 50%)",
              "loc": 7357
            },
            {
              "name": "pathB",
              "color": "hsl(32, 70%, 50%)",
              "children": [
                {
                  "name": "pathB1",
                  "color": "hsl(300, 70%, 50%)",
                  "loc": 180056
                },
                {
                  "name": "pathB2",
                  "color": "hsl(23, 70%, 50%)",
                  "loc": 145183
                },
                {
                  "name": "pathB3",
                  "color": "hsl(301, 70%, 50%)",
                  "loc": 125
                },
                {
                  "name": "pathB4",
                  "color": "hsl(335, 70%, 50%)",
                  "loc": 169796
                }
              ]
            },
            {
              "name": "pathC",
              "color": "hsl(273, 70%, 50%)",
              "children": [
                {
                  "name": "pathC1",
                  "color": "hsl(94, 70%, 50%)",
                  "loc": 40719
                },
                {
                  "name": "pathC2",
                  "color": "hsl(76, 70%, 50%)",
                  "loc": 190084
                },
                {
                  "name": "pathC3",
                  "color": "hsl(83, 70%, 50%)",
                  "loc": 82740
                },
                {
                  "name": "pathC4",
                  "color": "hsl(176, 70%, 50%)",
                  "loc": 43781
                },
                {
                  "name": "pathC5",
                  "color": "hsl(67, 70%, 50%)",
                  "loc": 82956
                },
                {
                  "name": "pathC6",
                  "color": "hsl(336, 70%, 50%)",
                  "loc": 36968
                },
                {
                  "name": "pathC7",
                  "color": "hsl(258, 70%, 50%)",
                  "loc": 123484
                },
                {
                  "name": "pathC8",
                  "color": "hsl(184, 70%, 50%)",
                  "loc": 21549
                },
                {
                  "name": "pathC9",
                  "color": "hsl(46, 70%, 50%)",
                  "loc": 151725
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}`

export default function VictoriaSunburst(props: SunburstProps) {
  if (!props.data) {
    return (<></>)
  }
  const stuff = JSON.parse(uwu);
  return (
    <div style={{ height: '500px', width: '500px' }}>
      <ResponsiveSunburst
        data={props.data}
        margin={{ top: 40, right: 20, bottom: 20, left: 20 }}
        id="name"
        value="loc"
        cornerRadius={2}
        borderWidth={1}
        borderColor="white"
        colors={{ scheme: 'nivo' }}
        childColor={{ from: 'color' }}
        animate={true}
        motionConfig="gentle"
        isInteractive={true}
      />
    </div>
  );
}