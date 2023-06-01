var times = [];
var rs = []

export function write_message(tau, r, current, max_len) {
    current = current + 1
    document.getElementById("message").innerText = "Calculating data points ... ["+current+"/"+max_len+"]"
    times.push(tau)
    rs.push(r)

    if (current >= max_len){
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
    }
}