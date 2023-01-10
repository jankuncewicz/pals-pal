import('csv-parse/lib/sync').then(csv => {
	parseFile = (file) => {
		const records = csv.parse(file, {delimiter: ','})
		times = new Float64Array(records.map(parseFloat))
		loadFileFlag = true
	}
}).catch(console.error);

import("../pkg/index.js").then(wasm => {
    function get(s){
    	return document.getElementById(s).value;
    }

	let fruits = new Map([
		["apples", 500],
		["bananas", 300],
		["oranges", 200]
	]);

    run = () => {
		let r, temp, delta, tau;
		switch(tabsState){
			case 0:
 	 	  		r = parseFloat(get("ans"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		tau = wasm.calculate_tau(r, delta, temp);
    			document.getElementById("tau").value = tau;
				console.log(fruits)
				wasm.test(fruits);
				break
			case 1:
 	 	  		tau = parseFloat(get("tau"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		r = wasm.calculate_r(tau, delta, temp);
    			document.getElementById("ans").value = r;
				break
			case 2:
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
				readFile().then(() => {
					let rs = new Float64Array(times)
					console.log(rs)
					wasm.calculate_array(rs, delta, temp)
					console.log(rs);
					console.log(times);

					ansString = []
					for (let i = 0; i < times.length; i++) {
						ansString.push(times[i] + "," + rs[i] + "\n")
					}

					const ans = new File(ansString, "calculated_rs.csv", {type: 'text/csv;charset=utf-8;'});
					const link = document.createElement('a')
					const url = URL.createObjectURL(ans)
				  
					link.href = url
					link.download = ans.name
					document.body.appendChild(link)
					link.click()
				  
					document.body.removeChild(link)
					window.URL.revokeObjectURL(url)
				})
				//wasm.calculate_array(times)
    }}

    // let chart = Chart.power("canvas", Number(2));
}).catch(console.error);
