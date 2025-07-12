<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  type Props = {
    values: [number, number][]; // [timestamp, value]
  };

  const { values }: Props = $props();

  let canvas: HTMLCanvasElement;
  let chart: Chart;

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString();
  };

  onMount(() => {
    chart = new Chart(canvas, {
      type: "line",
      data: {
        labels: values.map(() => ""),
        datasets: [
          {
            label: "Values",
            data: values.map(([, value]) => value),
            backgroundColor: "rgba(54, 162, 235, 0.2)",
            borderColor: "rgba(54, 162, 235, 1)",
            borderWidth: 2,
            tension: 0,
          },
        ],
      },
      options: {
        responsive: true,
        animation: false,
        plugins: {
          legend: {
            display: false,
          },
        },
        scales: {
          x: {
            display: false,
          },
          y: {
            min: 0,
            max: 2 * Math.PI,
          },
        },
      },
    });

    $effect(() => {
      if (chart && values) {
        chart.data.labels = values.map(() => "");
        chart.data.datasets[0].data = values.map(([, value]) => value);
        chart.update();
      }
    });

    return () => chart.destroy();
  });
</script>

<canvas bind:this={canvas}></canvas>

<style>
  canvas {
    max-width: 100%;
    height: auto;
  }
</style>
