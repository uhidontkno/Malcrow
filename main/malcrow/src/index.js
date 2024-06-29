
function ghProfileWindow() {
    let w = new __TAURI__.window.WebviewWindow('ghProfile', {
        url: 'https://github.com/uhidontkno',
      });
      w.show();
      (async () => {
        await w.setTitle('Malcrow | github.com/uhidontkno');
    });
}
function ghProjectWindow() {
    let w = new __TAURI__.window.WebviewWindow('ghProject', {
        url: 'https://github.com/uhidontkno/Malcrow',
      });
        w.show();
    (async () => {
        await w.setTitle('Malcrow | github.com/uhidontkno/Malcrow');
    });
}
document.querySelector(".profile").addEventListener("click",(e)=>{
    e.preventDefault();
})
document.querySelector(".project").addEventListener("click",(e)=>{
    e.preventDefault();
})