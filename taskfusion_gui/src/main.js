const { invoke } = window.__TAURI__.tauri;

let jsonContent;
var gettingpid=0;
//create an async function for get_system_info

// function colorCodeRow() {
//   const tableBody = document.getElementById("processes-body");
//   const rows = tableBody.getElementsByTagName("tr");
//   console.log("the number of rows is ", rows[0].getElementsByTagName("td")[18].textContent);

//   for (let i = 0; i < rows.length; i++) {
//     console.log("in for loooppp\n");
//     const cpu = parseFloat(rows[i].getElementsByTagName("td")[18].textContent);
//     const memory = parseFloat(rows[i].getElementsByTagName("td")[17].textContent);
//     let color = "";

//     if ((cpu >= 0 && cpu <= 5) || (memory >= 0 && memory <= 50)) {
//       color = "green";
//     } else if ((cpu >= 5 && cpu <= 15) || (memory >= 50 && memory <= 100)) {
//       color = "yellow";
//     } else if (cpu > 15 || memory > 100) {
//       color = "red";
//     }

//     rows[i].style.backgroundColor = color;
//   }
// }



$(document).ready(function() {
  // colorCodeRow();
  updateProgressBars();
  updateProcessTable();
  get_system_info();
  const contextMenu = $(".wrapper"),
    shareMenu = contextMenu.find(".share-menu");
  const card = $(".processes-table");

  card.on("contextmenu", "tr", function(e) {
    e.preventDefault();

    let x = e.clientX + window.scrollX,
    y = e.clientY + window.scrollY,
      cmWidth = contextMenu.width(),
      cmHeight = contextMenu.height();

    if (x + cmWidth > window.innerWidth) {
      x -= cmWidth;
    }

    if (y + cmHeight > window.innerHeight) {
      y -= cmHeight;
    }

    contextMenu.css({ left: x, top: y, visibility: "visible" });

    // Get the value of the first column in the current row
    gettingpid= $(this).find("td:eq(0)").html();
    
    // Do something with the value of col1
    console.log("the value of the column is ", gettingpid, "\n");
  });

  $(document).on("click", function() {
    contextMenu.css({ visibility: "hidden" });
  });
});

async function get_system_info() {
  //invoke the get_system_info command
  const result = await invoke("get_system_info");
  //check if result is not null or undefined
  if (result) {
    //log the result
    const parsedResult = JSON.parse(result);
    console.log(parsedResult);
    
    //take the result and change the system-info div in welcome.html but outline the html of each <p> tag
    document.getElementById("sys-name").innerHTML = parsedResult.system_name;
    document.getElementById("sys-kernel").innerHTML = parsedResult.kernel_version;
    document.getElementById("os-version").innerHTML = parsedResult.os_version;
    document.getElementById("host-name").innerHTML = parsedResult.hostname;
  } else {
    console.log("Result is null or undefined");
  }
}

//setInterval(get_system_info, 1000);//change 

async function omarlog(message) {
  await invoke("log_to_terminal", { message });
}


async function updateProgressBars() {
  const usage = await invoke("get_system_usage");
  const parsedResult = JSON.parse(usage);

  const memoryBar = document.getElementById("memory-bar");
  memoryBar.style.width = `${parsedResult.mem_percent}%`;

  const cpuBar = document.getElementById("cpu-bar");
  cpuBar.style.width = `${parsedResult.cpu_percent}%`;
  omarlog(cpuBar.style.width);

  const swapBar = document.getElementById("swap-bar");
  swapBar.style.width = `${parsedResult.swap_percent}%`;

  const memoryBarPercentage = document.getElementById("memory-bar-percentage");
  memoryBarPercentage.textContent = `${parsedResult.mem_percent}%`;

  const cpuBarPercentage = document.getElementById("cpu-bar-percentage");
  cpuBarPercentage.textContent = `${parsedResult.cpu_percent}%`;

  const swapBarPercentage = document.getElementById("swap-bar-percentage");
  swapBarPercentage.textContent = `${parsedResult.swap_percent}%`;

}

setInterval(updateProgressBars, 2500);

async function updateProcessTable() {
  let result;
  const filterBy = document.getElementById("filter-by").value;
  const sortBy = document.getElementById("sort-by").value;
  // invoke the sort_by command
  if (sortBy === "pid") {
    result = await invoke("sortby_pid");
  } else if (sortBy === "priority") {
    result = await invoke("sortby_priority");
  } else if (sortBy === "parent") {
    result = await invoke("sortby_parent");
  } else if (sortBy === "session") {
    result = await invoke("sortby_session");
  } else if (sortBy === "group") {
    result = await invoke("sortby_group");
  }else {
  result = await invoke("listprocesses");
  }
  
  const parsedProcesses = JSON.parse(result);

  const tableBody = document.getElementById("processes-body");
  tableBody.innerHTML = "";

  parsedProcesses.forEach(process => {
    const tr = document.createElement("tr");

    const pidTd = document.createElement("td");
    if(process.pid ==204916){
      console.log(process.priority);
    }
    pidTd.textContent = process.pid;
    tr.appendChild(pidTd);

    const stateTd = document.createElement("td");
    stateTd.textContent = process.state;
    tr.appendChild(stateTd);

    const ppidTd = document.createElement("td");
    ppidTd.textContent = process.ppid;
    tr.appendChild(ppidTd);

    const pgrpTd = document.createElement("td");
    pgrpTd.textContent = process.pgrp;
    tr.appendChild(pgrpTd);

    const sessionTd = document.createElement("td");
    sessionTd.textContent = process.session;
    tr.appendChild(sessionTd);

    const ttyTd = document.createElement("td");
    ttyTd.textContent = process.tty_nr;
    tr.appendChild(ttyTd);

    const tpgidTd = document.createElement("td");
    tpgidTd.textContent = process.tpgid;
    tr.appendChild(tpgidTd);

    const flagsTd = document.createElement("td");
    flagsTd.textContent = process.flags;
    tr.appendChild(flagsTd);

    const utimeTd = document.createElement("td");
    utimeTd.textContent = process.utime;
    tr.appendChild(utimeTd);

    const stimeTd = document.createElement("td");
    stimeTd.textContent = process.stime;
    tr.appendChild(stimeTd);

    const priorityTd = document.createElement("td");
    priorityTd.textContent = process.priority;
    tr.appendChild(priorityTd);

    const niceTd = document.createElement("td");
    niceTd.textContent = process.nice;
    tr.appendChild(niceTd);

    const numThreadsTd = document.createElement("td");
    numThreadsTd.textContent = process.num_threads;
    tr.appendChild(numThreadsTd);

    const startTimeTd = document.createElement("td");
    startTimeTd.textContent = process.starttime;
    tr.appendChild(startTimeTd);

    const vsizeTd = document.createElement("td");
    vsizeTd.textContent = process.vsize;
    tr.appendChild(vsizeTd);

    const cmdTd = document.createElement("td");
    cmdTd.textContent = process.cmd;
    tr.appendChild(cmdTd);

    const memUsageTd = document.createElement("td");
    memUsageTd.textContent = `${process.memory_usage_mb.toFixed(2)} MB`;
    tr.appendChild(memUsageTd);

    const cpuUsageTd = document.createElement("td");
    
    cpuUsageTd.textContent = `${process.cpu_usage.toFixed(2)}%`;
    tr.appendChild(cpuUsageTd);
    let calc = process.cpu_usage;
    if (calc > 10) {
      tr.style.backgroundColor = "rgba(255, 0, 0, 0.35)";
    } else if (calc >= 5 && calc <= 10) {
      tr.style.backgroundColor = "rgba(255, 255, 0, 0.35)";
    } else {
      tr.style.backgroundColor = "rgba(0, 255, 0, 0.35)";
    }
    console.log("the lenght of the cmd is ",cmdTd.length);


    tableBody.appendChild(tr);
  });

}

setInterval(updateProcessTable, 5000);

function showInput(select) {
  const filterBy = select.value;
  const inputContainer = document.getElementById('input-container');
  inputContainer.innerHTML = '';

  if (filterBy === 'pid' || filterBy === 'state'||filterBy==='cmd'||filterBy==='gid'||filterBy==='ppid') {
    const input = document.createElement('input');
    input.setAttribute('type', 'text');
    input.setAttribute('id', 'filter-value');
    input.setAttribute('placeholder', `Enter ${filterBy}`);
    inputContainer.appendChild(input);
  }
}
async function takeSnapshot() {
  const result = await invoke("takesnapshot");
}

async function filter_by() {
  // get the value of the filter by select
  const filterBy = document.getElementById("filter-by").value;
  // get the value of the filter value
  const filterValue = document.getElementById("filter-value").value;

  // invoke the filter_by command
  if(filterValue!=""){
  let result;  
  if (filterBy === "pid") {
    result = await invoke("filterByPid", { pid: parseInt(filterValue) });
  } else if (filterBy === "state") {
    result = await invoke("filterByState", { state: filterValue });
  } else if (filterBy === "ppid") {
    result = await invoke("filterByPpid", { ppid: parseInt(filterValue) });
  } else if (filterBy === "gid") {
    result = await invoke("filterByGID", { gid: parseInt(filterValue) });
  } else if(filterBy ==="cmd"){
    result = await invoke("filterby_cmd", {cmdyo: filterValue});
  }

  try{
  const parsedResult = JSON.parse(result);
  const tableBody = document.getElementById("processes-body");
  tableBody.innerHTML = "";

  parsedResult.forEach(process => {
    const tr = document.createElement("tr");

    const pidTd = document.createElement("td");
    pidTd.textContent = process.pid;
    tr.appendChild(pidTd);

    const stateTd = document.createElement("td");
    stateTd.textContent = process.state;
    tr.appendChild(stateTd);

    const ppidTd = document.createElement("td");
    ppidTd.textContent = process.ppid;
    tr.appendChild(ppidTd);

    const pgrpTd = document.createElement("td");
    pgrpTd.textContent = process.pgrp;
    tr.appendChild(pgrpTd);

    const sessionTd = document.createElement("td");
    sessionTd.textContent = process.session;
    tr.appendChild(sessionTd);

    const ttyTd = document.createElement("td");
    ttyTd.textContent = process.tty_nr;
    tr.appendChild(ttyTd);

    const tpgidTd = document.createElement("td");
    tpgidTd.textContent = process.tpgid;
    tr.appendChild(tpgidTd);

    const flagsTd = document.createElement("td");
    flagsTd.textContent = process.flags;
    tr.appendChild(flagsTd);

    const utimeTd = document.createElement("td");
    utimeTd.textContent = process.utime;
    tr.appendChild(utimeTd);

    const stimeTd = document.createElement("td");
    stimeTd.textContent = process.stime;
    tr.appendChild(stimeTd);

    const priorityTd = document.createElement("td");
    priorityTd.textContent = process.priority;
    tr.appendChild(priorityTd);

    const niceTd = document.createElement("td");
    niceTd.textContent = process.nice;
    tr.appendChild(niceTd);

    const numThreadsTd = document.createElement("td");
    numThreadsTd.textContent = process.num_threads;
    tr.appendChild(numThreadsTd);

    const startTimeTd = document.createElement("td");
    startTimeTd.textContent = process.starttime;
    tr.appendChild(startTimeTd);

    const vsizeTd = document.createElement("td");
    vsizeTd.textContent = process.vsize;
    tr.appendChild(vsizeTd);

    const cmdTd = document.createElement("td");
    cmdTd.textContent = process.cmd;
    tr.appendChild(cmdTd);

    const memUsageTd = document.createElement("td");
    memUsageTd.textContent = `${process.memory_usage_mb.toFixed(2)} MB`;
    tr.appendChild(memUsageTd);

    const cpuUsageTd = document.createElement("td");
    cpuUsageTd.textContent = `${process.cpu_usage.toFixed(2)}%`;
    tr.appendChild(cpuUsageTd);

    let calc = process.cpu_usage;
    if (calc > 10) {
      tr.style.backgroundColor = "rgba(255, 0, 0, 0.35)";
    } else if (calc >= 5 && calc <= 10) {
      tr.style.backgroundColor = "rgba(255, 255, 0, 0.35)";
    } else {
      tr.style.backgroundColor = "rgba(0, 255, 0, 0.35)";
    }

    tableBody.appendChild(tr);
  });
}
  catch(error){
    const tableBody=document.getElementById("processes-body");
    tableBody.innerHTML = "";
    return;
  }

  // update the process table with the filtered data
}

}

setInterval(filter_by, 5000);

// function showPrioritySubMenu() {
//   //get the priority sub-menu element
//   const prioritySubMenu = document.getElementById("priority-submenu");
//   //change the visibility of the priority sub-menu to visible
//   prioritySubMenu.style.visibility = "visible";
// }

async function changePriority(priority) {
  //const pid = document.getElementById("pid").textContent;
  //console.log("the pid i got is ", parseInt(gettingpid), " the priority is ", priority, "\n");
  const result = await invoke("changepriority", { pid: (gettingpid).toString(), priority: parseInt(priority).toString() });

}

async function killProcess() {
  const result = await invoke("process_kill", { pid: parseInt(gettingpid) });
}

async function killChildProcesses() {
  const result = await invoke("recursive_kill", { id:parseInt(gettingpid) });
}

async function pauseProcess() {
  const result = await invoke("pause_process", { id:parseInt(gettingpid) });
}

async function recursivePause() {
  const result = await invoke("recursive_pause", { id:parseInt(gettingpid) });
}

async function recursiveResume() {
  const result = await invoke("pause_resume", { id: parseInt(gettingpid) });
}

async function resumeProcess() {
  const result = await invoke("resume_process", { id: parseInt(gettingpid) });
}

//document.getElementById("filter-button").addEventListener("click", filter_by);

// Add an event listener to the document to hide the context menu when the user clicks outside of it

//create an event listener for get_system_info to run when the page loads but doens't close and open the application multiple times
