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


async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function get_random_num(){
  random_num.textContent = await invoke("get_random_num");
}

async function onload(){
  //获取列表信息
  list_1.textContent = await invoke("get_list",{num:1});
  list_2.textContent = await invoke("get_list",{num:2});
  list_3.textContent = await invoke("get_list",{num:3});
  list_4.textContent = await invoke("get_list",{num:4});
  list_5.textContent = await invoke("get_list",{num:5});

}

window.greet = greet;
window.get_random_num = get_random_num;
