import("../pkg/index.js").then(wasm => {
    test = wasm.calculate_r(21, 0.18, 295)
    console.log(wasm.calculate_r(21, 0.18, 295))
    var rs = [], taus = [];

    fetch("tauD.csv").then(resp => resp.text()).then(textData =>
    	textData.split("\n")
    		.map(e => {
    			let e1 = e.split(",").map(e2 => e2 = parseFloat(e2))
    			taus.push(e1[0])
    			rs.push(e1[1]/2)
    		}))

    taus.pop()
    console.log(rs)

    function get(s){
    	return document.getElementById(s).value;
    }

    run = () => {
    	let tau = parseFloat(get("tau"))
    	let temp = parseFloat(get("temp"))
    	let delta = parseFloat(get("delta"))
    	let r = wasm.calculate_r(tau, delta, temp);
    	document.getElementById("ans").value = r;
    }

    // let chart = Chart.power("canvas", Number(2));
}).catch(console.error);
