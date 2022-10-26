import('csv-parse').then(csv => {
	parseFile = file => {
		parser = csv.parse(file, {
			delimiter: ','
		}, (err, records) => {
			console.log(records)
		});
	}
}).catch(console.error);

import("../pkg/index.js").then(wasm => {

	//////////////// Testy na podstawie pliku tauD.csv ////////////////////////
	/*
    var rs = [], taus = [];

    fetch("tauD.csv").then(resp => resp.text()).then(textData =>
    	textData.split("\n")
    		.map(e => {
    			let e1 = e.split(",").map(e2 => e2 = parseFloat(e2))
    			taus.push(e1[0])
    			rs.push(e1[1]/2)
    		})).then(() => {
				rs = rs.slice(0, rs.length/2)
				taus = taus.slice(0, rs.length/3)
				
				for(r in taus){
					//rs[r] = [rs[r], wasm.calculate_tau(rs[r], 0.18, 295)]
					taus[r] = [taus[r], wasm.calculate_r(taus[r], 0.18, 295)]
				}
				console.log(taus)
			})
			*/
	///////////////////////////////////////////////////////////////////////////

    function get(s){
    	return document.getElementById(s).value;
    }

    run = () => {
		let r, temp, delta, tau;
		switch(tabsState){
			case 0:
 	 	  		r = parseFloat(get("ans"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		tau = wasm.calculate_tau(r, delta, temp);
    			document.getElementById("tau").value = tau;
				break
			case 1:
 	 	  		tau = parseFloat(get("tau"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		r = wasm.calculate_r(tau, delta, temp);
    			document.getElementById("ans").value = r;
				break
			case 2:
				readFile();
    }}

    // let chart = Chart.power("canvas", Number(2));
}).catch(console.error);
