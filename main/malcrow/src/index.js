let enabled = true;
let proc = [];
let reg = [];
function ghProfileWindow() {
    let w = new __TAURI__.window.WebviewWindow('ghProfile', {
        url: 'https://github.com/uhidontkno',
      });
      (async () => {
        await w.setTitle('Malcrow | github.com/uhidontkno');
        w.show();
    });
}
function ghProjectWindow() {
    let w = new __TAURI__.window.WebviewWindow('ghProject', {
        url: 'https://github.com/uhidontkno/Malcrow',
      });
    (async () => {
        await w.setTitle('Malcrow | github.com/uhidontkno/Malcrow');
        w.show();
    });
}
document.querySelector(".profile").addEventListener("click",(e)=>{
    e.preventDefault();
})
document.querySelector(".project").addEventListener("click",(e)=>{
    e.preventDefault();
})

let themes = ["mocha","latte","night","aqua"];
let themeDisplay = ["Catppuccin Mocha","Catppuccin Latte","Night","Aqua"];
for (let i = 0; i < themes.length; i++) {
document.querySelector(".themeDropdownContent").innerHTML += `<li>
      <input
        type="radio"
        name="theme-dropdown"
        class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
        aria-label="${themeDisplay[i]}"
        value="${themes[i]}" onclick="setTheme('${themes[i]}')" />
    </li>`
}

function setTheme(theme) {
    let themeName = theme;
    document.querySelector("html").setAttribute("data-theme",themeName);
    localStorage.setItem("theme",themeName);
}

if (localStorage.getItem("theme")) {
    setTheme(localStorage.getItem("theme"));
}

function toggleMalcrow() {
    if (enabled) {
        document.querySelector(".malcrowSection").style.zIndex = "-100";
        document.querySelector(".malcrowSection").style.opacity = "0.5";
        document.querySelector(".malcrowSection").style.cursor = "not-allowed";
        document.querySelector(".malcrowSection").style.position = "relative";
        enabled = false;
    } else {
        document.querySelector(".malcrowSection").style.zIndex = "1";
        document.querySelector(".malcrowSection").style.opacity = "1";
        document.querySelector(".malcrowSection").style.cursor = "pointer";
        document.querySelector(".malcrowSection").style.position = "default";
        enabled = true;
    }
}
let _saveConfig = window.__TAURI__.invoke("_save_config")
let getConfig = window.__TAURI__.invoke("_get_config")

function saveData() {
    proc = document.querySelector(".processes").value.split("\n");
    reg = document.querySelector(".registry").value.split("\n");
    malcrow = document.querySelector(".malcrowToggle").checked;
    let config = {
        "malcrow": enabled,
        "proc": proc,
        "reg": reg
    }
    _saveConfig(config);

}

let config = getConfig();
if (!config["malcrow"]) {
    toggleMalcrow();
}
if (config["proc"]) {
    document.querySelector(".processes").innerText = config["proc"].join("\n");
}
if (config["reg"]) {
    document.querySelector(".registry").innerText = config["reg"].join("\n");
}

function addProc() {
    let procName = document.querySelector(".procInput").value;
    let prEle = document.querySelector(".processes");
    if (!proc.includes(procName)) {
        proc.push(procName)
    }
    prEle.innerText = proc.join("\n")
    document.querySelector(".procInput").value = "";
}
function remProc() {
    let procName = document.querySelector(".procInput").value;
    let prEle = document.querySelector(".processes");
    if (proc.includes(procName)) {
        proc.splice(proc.indexOf(procName),1)
    }
    prEle.innerText = proc.join("\n")
    document.querySelector(".procInput").value = "";
}
function addReg() {
    let regName = document.querySelector(".regInput").value;
    let regEle = document.querySelector(".registry");
    if (!reg.includes(regName)) {
        reg.push(regName)
    }
    regEle.innerText = reg.join("\n")
    document.querySelector(".regInput").value = "";
}
function remReg() {
    let procName = document.querySelector(".procInput").value;
    let prEle = document.querySelector(".processes");
    if (proc.includes(procName)) {
        proc.splice(proc.indexOf(procName),1)
    }
    prEle.innerText = proc.join("\n")
    document.querySelector(".procInput").value = "";
}