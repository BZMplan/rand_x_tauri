const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

let random_num;

let list_1;
let list_2;
let list_3;
let list_4;
let list_5;

//映射资源
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  random_num = document.querySelector("#random_num");
  //更新列表
  list_1 = document.querySelector("#list_1");
  list_2 = document.querySelector("#list_2");
  list_3 = document.querySelector("#list_3");
  list_4 = document.querySelector("#list_4");
  list_5 = document.querySelector("#list_5");
});

//网页启动时自动运行程序
window.onload = onload;

// 条形统计图代码
const ctx = document.getElementById("students_chart");

//条形统计图代码

async function renew() {
  //更新列表信息
  // list_1.textContent = await invoke("get_list",{index:1});
  // list_2.textContent = await invoke("get_list",{index:2});
  // list_3.textContent = await invoke("get_list",{index:3});
  // list_4.textContent = await invoke("get_list",{index:4});
  // list_5.textContent = await invoke("get_list",{index:5});
  //更新条形统计图信息'
  let s1 = await invoke("get_name",{index:1});
  let s2 = await invoke("get_name",{index:2});
  let s3 = await invoke("get_name",{index:3});
  let s4 = await invoke("get_name",{index:4});
  let s5 = await invoke("get_name",{index:5});
  let s6 = await invoke("get_name",{index:6});
  let s7 = await invoke("get_name",{index:7});
  let s8 = await invoke("get_name",{index:8});
  let s9 = await invoke("get_name",{index:9});
  let s10 = await invoke("get_name",{index:10});
  let c1 = await invoke("get_count",{index:1});
  let c2 = await invoke("get_count",{index:2});
  let c3 = await invoke("get_count",{index:3});
  let c4 = await invoke("get_count",{index:4});
  let c5 = await invoke("get_count",{index:5});
  let c6 = await invoke("get_count",{index:6});
  let c7 = await invoke("get_count",{index:7});
  let c8 = await invoke("get_count",{index:8});
  let c9 = await invoke("get_count",{index:9});
  let c10 = await invoke("get_count",{index:10});

  new Chart(ctx, {
    type: "bar",
    data: {
      labels: [s1, s2, s3, s4, s5, s6,s7,s8,s9,s10],
      datasets: [
        {
          label: "抽取次数",
          data: [c1, c2, c3, c4, c5, c6,c7,c8,c9,c10],
          borderWidth: 1,
        },
      ],
    },
    options: {
      scales: {
        y: {
          beginAtZero: true,
        },
      },
    },
  });
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function get_random_num() {
  random_num.textContent = await invoke("get_random_num");
  renew();
}

async function onload() {
  renew();
}



window.greet = greet;
window.get_random_num = get_random_num;
