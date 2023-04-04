import {type ChartOptions} from 'chart.js';

export const getLineChartOptions = (
  {minLimit, maxLimit, chartTitle, xAxisTitle, yAxisTitle}: {
    minLimit: number;
    maxLimit: number;
    chartTitle: string;
    xAxisTitle: string;
    yAxisTitle: string;}) => {
  const minZoomRangeMillis = 60 * 1000;
  const options: ChartOptions<'line'> = {
    showLine: true,
    animation: false,
    parsing: false,
    interaction: {
      mode: 'point',
      axis: 'x',
      intersect: false,
    },
    indexAxis: 'x',
    plugins: {
      tooltip: {
        displayColors: false,
      },

      decimation: {
        enabled: true,
        algorithm: 'lttb',
        threshold: 100,
        samples: 100,
      },

      title: {
        display: true,
        text: chartTitle,
      },
      zoom: {
        pan: {
          enabled: true,
          mode: 'x',
        },
        limits: {
          x: {
            minRange: minZoomRangeMillis,
            min: minLimit,
            max: maxLimit,
          },
          y: {
            min: 'original',
            max: 'original',
          },
        },
        zoom: {
          wheel: {
            enabled: true,
            speed: 0.2,
          },
          pinch: {
            enabled: true,
          },
          mode: 'x',
        },
      },
    },
    responsive: true,
    scales: {
      y: {
        title: {
          display: true,
          text: yAxisTitle,
        },
        min: 0,
      },
      x: {
        title: {
          display: true,
          text: xAxisTitle,
        },
        min: minLimit,
        max: maxLimit,
        ticks: {
          source: 'auto',
        },
        type: 'time',
        time: {
          displayFormats: {
            hour: 'HH:mm',
            minute: 'HH:mm',
            second: 'HH:mm:ss',
          },
        },
      },
    },
  };
  return options;
};
