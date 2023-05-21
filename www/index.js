import init, { calculate_array, calculate_r, calculate_tau, Chart } from "../pals-pal/pkg/pals_pal.js";

function get(s){
	return document.getElementById(s).value;
}

//TODO: implement csv parsing
async function run_wasm(){
	await init();
    run = () => {
		let r, temp, delta, tau;
		switch(tabsState){
			case 0:
 	 	  		r = parseFloat(get("ans"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		tau = calculate_tau(r, delta, temp);
    			document.getElementById("tau").value = tau;
				break
			case 1:
 	 	  		tau = parseFloat(get("tau"))
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
	    		r = calculate_r(tau, delta, temp);
    			document.getElementById("ans").value = r;
				break
			case 2:
	   		 	temp = parseFloat(get("temp"))
	    		delta = parseFloat(get("delta"))
				readFile().then((times) => {
					let rs = new Float64Array(times)
					console.log(rs)
					calculate_array(rs, delta, temp, "canvas")
					console.log(rs);
					console.log(times);

					let ansString = []
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
    //let chart = Chart.power("canvas", Number(2));
}
run_wasm();
