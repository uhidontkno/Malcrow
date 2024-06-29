
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