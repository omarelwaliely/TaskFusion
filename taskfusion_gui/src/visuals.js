const { invoke } = window.__TAURI__.tauri;

async function omarlog(message) {
    await invoke("log_to_terminal", { message });
}
async function getSystemUsage() {
    const result = await invoke("get_system_usage");
    return JSON.parse(result);
}
async function getProgramUsage() {
    const result = await invoke("listprocesses");
    return JSON.parse(result);
}

//////////////////pie chart////////////////////////

setInterval(createPieUsageChart, 5000);
setInterval(createSystemUsageChart, 1000);

const piechartData = {
    labels: [],
    datasets: [
      {
        label: 'Memory Usage',
        data: [],
        backgroundColor: [
          'rgba(255, 0, 0, 1)',
          'rgba(0, 255, 0, 1)',
          'rgba(0, 0, 255, 1)',
          'rgba(0, 0, 0, 1)',
          '#FF69B4',
          '#FF6F00',
          '#9A00FF',
          '#EFFF00',
          '#00E6FF',
          '#964B00',
        ],
        borderColor: [
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
          '#000000',
        ],
        borderWidth: 1,
      },
    ],
  };


const piechartOptions = {
    animation: false,
};

const piechartConfig = {
    type: 'pie',
    data: piechartData,
    options: piechartOptions, 
};

const piectx = document.getElementById('pie-chart').getContext('2d');
const piechart = new Chart(piectx, piechartConfig,);



async function createPieUsageChart() {
    const processes = await getProgramUsage();
    processes.sort((a, b) => b.memory_usage_mb - a.memory_usage_mb);
    const numProcesses = Math.min(10,processes.length); // only display the top 10 processes
    const currentDataset = piechartData.datasets[0].data;
    const labels = piechartData.labels;
    currentDataset.length = 0;
    labels.length = 0;
 
    for (let i = 0; i < numProcesses; i++) {
      const process = processes[i];
      const UsagePercent = (process.memory_usage_mb) ;
      currentDataset.push(UsagePercent.toFixed(2));
      labels.push(process.cmd);
    }
   //console.log(processes[0].cmd + processes[0].cpu_usage);

    piechart.update();
}

/////////////system chart//////////////////////////





const chartData = {
    labels: Array.from({ length: 60 }, (_, i) => i + 1),
    datasets: [
        {
            label: 'Memory',
            data: [],
            backgroundColor: 'rgba(162, 0, 255, 0.2)',
            borderColor: 'rgba(162, 0, 255, 1)',
            borderWidth: 3,
            fill: false
            
        },
        {
            label: 'Swap',
            data: [],
            backgroundColor: 'rgba(94, 216, 106, 0.2)',
            borderColor: 'rgba(94, 216, 106, 1)',
            borderWidth: 3,
            fill: false
        },
        {
            label: 'CPU',
            data: [],
            backgroundColor: 'rgba(255, 100, 100, 0.2)',
            borderColor: 'rgba(255, 100, 100, 1)',
            borderWidth: 3,
            fill: false
        }
    ]
};

const chartOptions = {
    elements: {
        point: {
            radius: 1.5
        },
        line: {
            borderWidth: 0
        }
    },

    scales: {
        xAxes: {
            ticks: {
                display: false,
                beginAtZero: true
            }
        },
        y: {
            beginAtZero: true,
            title: {
                display: true,
                label: 'Percentage'
            },
            ticks: {
                precision: 10
            },
            grid: {
                display: false
            }
        }
    },
};

const chartConfig = {
    type: 'line',
    data: chartData,
    options: chartOptions, 
};

const ctx = document.getElementById('system-chart').getContext('2d');
const chart = new Chart(ctx, chartConfig,);


async function createSystemUsageChart() {
    const data = await getSystemUsage();
    const memoryDataset = chartData.datasets[0].data;
    const swapDataset = chartData.datasets[1].data;
    const cpuDataset = chartData.datasets[2].data;
    memoryDataset.push(data.mem_percent);
    swapDataset.push(data.swap_percent);
    cpuDataset.push(data.cpu_percent);
    if (memoryDataset.length > 60) {
        memoryDataset.shift();
    }
    if (swapDataset.length > 60) {
        swapDataset.shift();
    }
    if (cpuDataset.length > 60) {
        cpuDataset.shift();
    }
    chart.update();
}
document.addEventListener("DOMContentLoaded", function() {
    console.log("inmainjs");
   createPieUsageChart();
   createSystemUsageChart();
  }
  );

// const backButton = document.getElementById('back-button');
// backButton.addEventListener('click', () => {
//   tauri.navigateBack();
// }

// );









