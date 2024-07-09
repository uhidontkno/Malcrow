let addresses = ["186dGbqMAFLPzQAbdzaYtAWDiQwgDPFq8y",
    "0x4913AAb84c4b5f91753Ea517d372F0662e70CEEf",
    "LTaj6GCGJXQMtVcX1XtTeDWN4Yt2oaZTQ6",
    "49LFZRV4mdFgL5S8Tgr94g96L5dwE7Lx9b9ouetbFxwNKLpuL8y7CbBHMJjVv9KefLVh8uejfiNZiZFEVATqREDKAufEnqL",
    "DTpTrzDSDjswGhFcGm3Eaq832grWDpAAZD",]
    function selEvent() {
        (async()=>{
        document.querySelector(".paymentSection .addy").innerText = addresses[document.querySelector(".cryptoSelect").selectedIndex - 1]
        let price = `${await getCryptoPricing(document.querySelector(".cryptoSelect").value.toLowerCase(),5)} ${document.querySelector(".cryptoSelect").value}`
        document.querySelector(".amtTitle").innerText = price
        document.querySelector(".amt").innerText = price
    })();
    }
    function numEvent() {
        (async()=>{
            let price = `${await getCryptoPricing(document.querySelector(".cryptoSelect").value.toLowerCase(),document.querySelector(".paymentSection .amtSend").value)} ${document.querySelector(".cryptoSelect").value}`
        document.querySelector(".amtTitle").innerText = price
        document.querySelector(".amt").innerText = price
        })();
    }
            function setTheme(theme) {
        let themeName = theme;
        document.querySelector("html").setAttribute("data-theme",themeName);
        localStorage.setItem("theme",themeName);
    }
    
    if (localStorage.getItem("theme")) {
        setTheme(localStorage.getItem("theme"));
    }
            async function getCryptoPricing(currency,amount) {
            let res = await (await fetch(`https://www.forbes.com/advisor/money-transfer/currency-converter/usd-${currency.toLowerCase()}/?amount=${amount}`)).text()
            return res.split(`<span class="amount">`)[1].split("\n")[0].replaceAll("</span>","").replaceAll("<span>","")
            }
        function copy(ele) {
            navigator.clipboard.writeText(ele.innerText);
            alert("Copied to clipboard.")
        }